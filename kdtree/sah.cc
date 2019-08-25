#include "kdtree/sah.h"

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include <algorithm>
#include <cassert>
#include <vector>

#ifndef NDEBUG
#include <cstdlib>
#endif

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "geometry/split.h"
#include "kdtree/intersect.h"
#include "kdtree/kdtree.h"
#include "kdtree/sah_cost.h"
#include "util/vector.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

using geometry::Aabb;
using geometry::AabbSplit;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using glm::vec3;
using kdtree::Cost;
using kdtree::IntersectResults;
using kdtree::KdNode;
using std::vector;

namespace {

constexpr unsigned int MAX_DEPTH = 20;

struct Event {
  enum Type { END, PLANAR, START };
  float distance;
  Type type;

  bool operator<(const Event& other) const {
    return distance < other.distance ||
           (distance == other.distance && type < other.type);
  }
};

void ListSplits(const Aabb& boundary,
                const Triangle& triangle,
                Axis axis,
                vector<Event>* splits) {
  assert(boundary.GetVolume() > 0.0f);
  float a = boundary.GetClamped(triangle.GetMin())[axis];
  float b = boundary.GetClamped(triangle.GetMax())[axis];
  if (a == b) {
    splits->push_back({a, Event::PLANAR});
  } else {
    splits->push_back({a, Event::START});
    splits->push_back({b, Event::END});
  }
}

struct KdBox {
  Aabb boundary;
  vector<const Triangle*> triangles;
};

/**
 * List perfect splits for a set of triangles.
 *
 * For each triangle there will be two events (or one if it is planar in the
 * chosen axis). No events are filtered away because then the triangle
 * associated with the filtered events would not be represented in calculations
 * that use these results.
 */
vector<Event> ListSplits(const KdBox& parent, Axis axis) {
  vector<Event> splits;
  for (const Triangle* triangle : parent.triangles) {
    ListSplits(parent.boundary, *triangle, axis, &splits);
  }
  std::sort(splits.begin(), splits.end());
  return splits;
}

struct EventCount {
  size_t pminus, pplus, pplane;
};

EventCount CountEvents(vector<Event>::const_iterator begin,
                       vector<Event>::const_iterator end) {
  assert(begin != end);
  size_t pminus = 0;
  size_t pplus = 0;
  size_t pplane = 0;
  auto iter = begin;
  while (iter != end && iter->distance == begin->distance &&
         iter->type == Event::END) {
    pminus += 1;
    ++iter;
  }
  while (iter != end && iter->distance == begin->distance &&
         iter->type == Event::PLANAR) {
    pplane += 1;
    ++iter;
  }
  while (iter != end && iter->distance == begin->distance &&
         iter->type == Event::START) {
    pplus += 1;
    ++iter;
  }
  return EventCount{pminus, pplus, pplane};
}

struct Split {
  geometry::Aap plane;
  Cost cost;
  bool operator<(const Split& other) const { return cost < other.cost; }
};

Split FindBestSplit(const KdBox& parent) {
  assert(parent.boundary.GetVolume() > 0.0f);
  assert(!parent.triangles.empty());

  Split best{{geometry::X, 0}, {FLT_MAX, kdtree::LEFT}};
  for (int axis_index = 0; axis_index < 3; ++axis_index) {
    Axis axis = static_cast<Axis>(axis_index);
    size_t number_left = 0;
    size_t number_right = parent.triangles.size();
    vector<Event> splits = ListSplits(parent, axis);
    for (auto iter = splits.cbegin(); iter != splits.cend(); ++iter) {
      EventCount count = CountEvents(iter, splits.cend());
      number_right = number_right - count.pminus - count.pplane;
      Aap plane(axis, iter->distance);
      Cost cost = kdtree::CalculateSahCost(parent.boundary, plane, number_left,
                                           number_right, count.pplane);
      Split split{plane, cost};
#ifndef NDEBUG
      printf("  FindBestSplit: Aap{%d, %f} cost=%f\n", axis,
             static_cast<double>(iter->distance),
             static_cast<double>(split.cost.cost));
#endif
      best = std::min(best, split);
      number_left = number_left + count.pplus + count.pplane;
    }
  }
#ifndef NDEBUG
  printf("FindBestSplit({%f, %lu}) = {Aap{%d, %f}, cost=%f, side=%d}\n",
         static_cast<double>(parent.boundary.GetVolume()),
         parent.triangles.size(), best.plane.GetAxis(),
         static_cast<double>(best.plane.GetDistance()),
         static_cast<double>(best.cost.cost), best.cost.side);
#endif

  return best;
}

KdNode* BuildHelper(unsigned int depth, const KdBox& parent) {
  assert(parent.boundary.GetVolume() > 0.0f);

#ifndef NDEBUG
  printf(
      "BuildHelper("
      "depth=%d, "
      "Aabb{(%f, %f, %f), (%f, %f, %f)}, "
      "triangles=%lu)\n",
      depth, static_cast<double>(parent.boundary.GetMin().x),
      static_cast<double>(parent.boundary.GetMin().y),
      static_cast<double>(parent.boundary.GetMin().z),
      static_cast<double>(parent.boundary.GetMax().x),
      static_cast<double>(parent.boundary.GetMax().y),
      static_cast<double>(parent.boundary.GetMax().z), parent.triangles.size());
#endif

  if (depth >= MAX_DEPTH || parent.triangles.empty()) {
    return new KdNode(new vector<const Triangle*>(parent.triangles));
  }

  Split best = FindBestSplit(parent);
  AabbSplit aabbs = geometry::Split(parent.boundary, best.plane);
  IntersectResults triangles =
      kdtree::PartitionTriangles(parent.boundary, parent.triangles, best.plane);
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

}  // namespace

namespace kdtree {
KdTree build(const vector<Triangle>& triangles) {
  vector<const Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  Aabb boundary = geometry::find_bounding(triangles);
  return KdTree(BuildHelper(0, KdBox{boundary, ptrs}));
}
}  // namespace kdtree
