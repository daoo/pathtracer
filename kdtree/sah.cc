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
#include "util/vector.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

using geometry::Aabb;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using glm::vec3;
using kdtree::IntersectResults;
using kdtree::KdNode;
using std::vector;

namespace {

constexpr unsigned int MAX_DEPTH = 20;

constexpr float COST_EMPTY_FACTOR = 0.8f;
constexpr float COST_TRAVERSE = 0.1f;
constexpr float COST_INTERSECT = 1.0f;

float CalculateCost(float probability_left,
                    float probability_right,
                    size_t number_left,
                    size_t number_right) {
  assert(probability_left >= 0);
  assert(probability_right >= 0);
  assert(probability_left > 0 || probability_right > 0);
  float empty_factor =
      number_left == 0 || number_right == 0 ? COST_EMPTY_FACTOR : 1.0f;
  float traverse_cost = COST_TRAVERSE;
  float intersect_cost = COST_INTERSECT * (probability_left * number_left +
                                           probability_right * number_right);
  return empty_factor * (traverse_cost + intersect_cost);
}

enum Side { LEFT, RIGHT };

struct SplitCost {
  Aap plane;
  float cost;
  Side side;

  bool operator<(const SplitCost& other) const { return cost < other.cost; }
};

SplitCost CalculateCost(const Aabb& parent,
                        const Aap& plane,
                        size_t left_count,
                        size_t right_count,
                        size_t plane_count) {
  assert(parent.GetSurfaceArea() > 0.0f);
  geometry::AabbSplit split = geometry::Split(parent, plane);
  if (split.left.GetVolume() <= 0.0f) return SplitCost{plane, FLT_MAX, LEFT};
  if (split.right.GetVolume() <= 0.0f) return SplitCost{plane, FLT_MAX, RIGHT};

  float surface_area_parent = parent.GetSurfaceArea();
  float surface_area_left = split.left.GetSurfaceArea();
  float surface_area_right = split.right.GetSurfaceArea();

  float probability_left = surface_area_left / surface_area_parent;
  float probability_right = surface_area_right / surface_area_parent;

  float cost_plane_left = CalculateCost(probability_left, probability_right,
                                        left_count + plane_count, right_count);
  float cost_plane_right = CalculateCost(probability_left, probability_right,
                                         left_count, right_count + plane_count);

  return cost_plane_left <= cost_plane_right
             ? SplitCost{plane, cost_plane_left, LEFT}
             : SplitCost{plane, cost_plane_right, RIGHT};
}

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

SplitCost FindBestSplit(const KdBox& parent) {
  assert(parent.boundary.GetVolume() > 0.0f);
  assert(!parent.triangles.empty());

  SplitCost best{{geometry::X, 0}, FLT_MAX, LEFT};
  for (int axis_index = 0; axis_index < 3; ++axis_index) {
    Axis axis = static_cast<Axis>(axis_index);
    size_t number_left = 0;
    size_t number_right = parent.triangles.size();
    vector<Event> splits = ListSplits(parent, axis);
    for (auto iter = splits.cbegin(); iter != splits.cend(); ++iter) {
      EventCount count = CountEvents(iter, splits.cend());
      number_right = number_right - count.pminus - count.pplane;
      Aap plane(axis, iter->distance);
      SplitCost split = CalculateCost(parent.boundary, plane, number_left,
                                      number_right, count.pplane);
#ifndef NDEBUG
      printf("  FindBestSplit: Aap{%d, %f} cost=%f\n", axis,
             static_cast<double>(iter->distance),
             static_cast<double>(split.cost));
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
         static_cast<double>(best.cost), best.side);
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

  SplitCost best = FindBestSplit(parent);
  geometry::AabbSplit aabbs = geometry::Split(parent.boundary, best.plane);
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
