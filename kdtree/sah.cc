#include "kdtree/sah.h"

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include <algorithm>
#include <cassert>
#include <set>
#include <vector>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "geometry/split.h"
#include "kdtree/intersect.h"
#include "kdtree/kdtree.h"
#include "util/vector.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

using geometry::Aabb;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using kdtree::IntersectResults;
using kdtree::KdNode;
using std::set;
using std::vector;

namespace {

constexpr unsigned int MAX_DEPTH = 20;

constexpr float COST_EMPTY = 0.01f;
constexpr float COST_TRAVERSE = 0.1f;
constexpr float COST_INTERSECT = 1.0f;

inline float LeafCostBound(size_t parent_count) {
  return COST_INTERSECT * parent_count;
}

inline float CalculateCost(float parent_area,
                           float left_area,
                           float right_area,
                           size_t left_count,
                           size_t right_count) {
  assert(parent_area > 0.0f);
  assert(left_area > 0.0f);
  assert(right_area > 0.0f);
  float traverse =
      left_count == 0 || right_count == 0 ? COST_EMPTY : COST_TRAVERSE;
  float area_heuristic = (left_area * left_count + right_area * right_count);
  float intersect = COST_INTERSECT * area_heuristic / parent_area;
  return traverse + intersect;
}

enum Side { LEFT, RIGHT };

struct SplitCost {
  Aap plane;
  float cost;
  Side side;

  bool operator<(const SplitCost& other) const { return cost < other.cost; }
};

inline SplitCost CalculateCost(const Aabb& parent,
                               const Aap& plane,
                               size_t left_count,
                               size_t right_count,
                               size_t plane_count) {
  geometry::AabbSplit split = geometry::Split(parent, plane);
  assert(split.left.GetVolume() > 0.0f);
  assert(split.right.GetVolume() > 0.0f);

  float parent_area = parent.GetSurfaceArea();
  float left_area = split.left.GetSurfaceArea();
  float right_area = split.right.GetSurfaceArea();

  float plane_left = CalculateCost(parent_area, left_area, right_area,
                                   left_count + plane_count, right_count);
  float plane_right = CalculateCost(parent_area, left_area, right_area,
                                    left_count, right_count + plane_count);

  return plane_left <= plane_right ? SplitCost{plane, plane_left, LEFT}
                                   : SplitCost{plane, plane_right, RIGHT};
}

enum Type { START, PLANAR, END };

struct Event {
  float distance;
  Type type;

  bool operator<(const Event& other) const {
    return distance < other.distance ||
           (distance == other.distance && type < other.type);
  }
};

inline void ListPerfectSplits(const Aabb& boundary,
                              const Triangle& triangle,
                              Axis axis,
                              set<Event>* splits) {
  assert(boundary.GetVolume() > 0.0f);
  float a = boundary.GetClamped(triangle.GetMin())[axis];
  float b = boundary.GetClamped(triangle.GetMax())[axis];
  if (a == b) {
    splits->insert({a, PLANAR});
  } else {
    splits->insert({a, START});
    splits->insert({b, END});
  }
}

struct KdBox {
  Aabb boundary;
  vector<const Triangle*> triangles;
};

inline set<Event> ListPerfectSplits(const KdBox& parent, Axis axis) {
  set<Event> splits;
  for (const Triangle* triangle : parent.triangles) {
    ListPerfectSplits(parent.boundary, *triangle, axis, &splits);
  }
  return splits;
}

struct EventCount {
  size_t pminus, pplus, pplane;
};

EventCount CountEvents(set<Event>::const_iterator begin,
                       set<Event>::const_iterator end) {
  assert(begin != end);
  size_t pminus = 0;
  size_t pplus = 0;
  size_t pplane = 0;
  auto iter = begin;
  while (iter != end && iter->distance == begin->distance &&
         iter->type == END) {
    pminus += 1;
    ++iter;
  }
  while (iter != end && iter->distance == begin->distance &&
         iter->type == PLANAR) {
    pplane += 1;
    ++iter;
  }
  while (iter != end && iter->distance == begin->distance &&
         iter->type == START) {
    pplus += 1;
    ++iter;
  }
  return EventCount{pminus, pplus, pplane};
}

SplitCost FindBestSplit(const KdBox& parent) {
  assert(parent.boundary.GetVolume() > 0.0f);
  assert(!parent.triangles.empty());
  SplitCost best{{geometry::X, 0}, FLT_MAX, LEFT};
  for (int axis_index = 0; axis_index < 3; ++axis_index) {
    Axis axis = static_cast<Axis>(axis_index);
    size_t nl = 0;
    size_t nr = parent.triangles.size();
    set<Event> splits = ListPerfectSplits(parent, axis);
    for (auto iter = splits.cbegin(); iter != splits.cend(); ++iter) {
      EventCount count = CountEvents(iter, splits.cend());
      nr = nr - count.pminus - count.pplane;
      Aap plane(axis, iter->distance);
      SplitCost split =
          CalculateCost(parent.boundary, plane, nl, nr, count.pplane);
      best = std::min(best, split);
      nl = nl + count.pplus + count.pplane;
    }
  }
  return best;
}

KdNode* BuildHelper(unsigned int depth, const KdBox& parent) {
  assert(parent.boundary.GetVolume() > 0.0f);
  if (depth >= MAX_DEPTH || parent.triangles.empty()) {
    return new KdNode(new vector<const Triangle*>(parent.triangles));
  }

  SplitCost best = FindBestSplit(parent);
  if (best.cost > LeafCostBound(parent.triangles.size())) {
    return new KdNode(new vector<const Triangle*>(parent.triangles));
  } else {
    geometry::AabbSplit aabbs = geometry::Split(parent.boundary, best.plane);
    IntersectResults triangles = kdtree::PartitionTriangles(
        parent.boundary, parent.triangles, best.plane);
    vector<const Triangle*> left_tris(triangles.left);
    vector<const Triangle*> right_tris(triangles.right);
    // Put plane-triangles on side with fewest triangels, or left if both equal.
    if (triangles.left.size() <= triangles.right.size()) {
      util::append(&left_tris, triangles.plane);
    } else {
      // triangles.left.size() > triangles.right.size()
      util::append(&right_tris, triangles.plane);
    }
    KdBox left{aabbs.left, left_tris};
    KdBox right{aabbs.right, right_tris};
    return new KdNode(best.plane, BuildHelper(depth + 1, left),
                      BuildHelper(depth + 1, right));
  }
}

}  // namespace

namespace kdtree {
KdTree build(const vector<Triangle>& triangles) {
  vector<const Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  return KdTree(
      BuildHelper(0, KdBox{geometry::find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
