#ifndef KDTREE_SAH_FAST_IMPL_H_
#define KDTREE_SAH_FAST_IMPL_H_

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include <cassert>

#include <set>
#include <vector>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "geometry/split.h"
#include "kdtree/build_common.h"
#include "kdtree/kdtree.h"
#include "kdtree/sah_common.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

enum Type { START, PLANAR, END };

struct Event {
  geometry::Aap plane;
  Type type;

  bool operator<(const Event& other) const {
    return plane < other.plane || (plane == other.plane && type < other.type);
  }
};

void ListPerfectSplits(const geometry::Aabb& boundary,
                       const geometry::Triangle& triangle,
                       geometry::Axis axis,
                       std::set<Event>* splits) {
  float boundary_min = boundary.GetMin()[axis];
  float boundary_max = boundary.GetMax()[axis];
  float triangle_min = triangle.GetMin()[axis];
  float triangle_max = triangle.GetMax()[axis];
  float clamped_min = glm::clamp(triangle_min, boundary_min, boundary_max);
  float clamped_max = glm::clamp(triangle_max, boundary_min, boundary_max);
  if (clamped_min == clamped_max) {
    splits->insert({{axis, clamped_min}, PLANAR});
  } else {
    splits->insert({{axis, clamped_min}, START});
    splits->insert({{axis, clamped_max}, END});
  }
}

void ListPerfectSplits(const geometry::Aabb& boundary,
                       const geometry::Triangle& triangle,
                       std::set<Event>* splits) {
  ListPerfectSplits(boundary, triangle, geometry::X, splits);
  ListPerfectSplits(boundary, triangle, geometry::Y, splits);
  ListPerfectSplits(boundary, triangle, geometry::Z, splits);
}

std::set<Event> ListPerfectSplits(const kdtree::KdBox& parent) {
  std::set<Event> splits;
  for (const geometry::Triangle* triangle : parent.triangles) {
    ListPerfectSplits(parent.boundary, *triangle, &splits);
  }
  return splits;
}

struct EventCount {
  size_t pminus, pplus, pplane;
};

EventCount CountEvents(std::set<Event>::const_iterator begin,
                       std::set<Event>::const_iterator end) {
  assert(begin != end);
  size_t pminus = 0;
  size_t pplus = 0;
  size_t pplane = 0;
  float distance = begin->plane.GetDistance();
  geometry::Axis axis = begin->plane.GetAxis();
  auto iter = begin;
  while (iter != end && iter->plane.GetDistance() == distance &&
         iter->plane.GetAxis() == axis && iter->type == END) {
    pminus += 1;
    ++iter;
  }
  while (iter != end && iter->plane.GetDistance() == distance &&
         iter->plane.GetAxis() == axis && iter->type == PLANAR) {
    pplane += 1;
    ++iter;
  }
  while (iter != end && iter->plane.GetDistance() == distance &&
         iter->plane.GetAxis() == axis && iter->type == START) {
    pplus += 1;
    ++iter;
  }
  return EventCount{pminus, pplus, pplane};
}

kdtree::KdSplit FindBestSplit(const kdtree::KdBox& parent,
                              const std::set<Event>& splits) {
  assert(splits.size() > 0);
  kdtree::KdSplit best{{geometry::X, 0}, {FLT_MAX, kdtree::LEFT}};
  for (int axis_index = 0; axis_index < 3; ++axis_index) {
    geometry::Axis axis = static_cast<geometry::Axis>(axis_index);
    size_t nl = 0;
    size_t nr = parent.triangles.size();
    for (auto iter = splits.cbegin(); iter != splits.cend(); ++iter) {
      if (iter->plane.GetAxis() == axis) {
        EventCount count = CountEvents(iter, splits.cend());
        nr = nr - count.pminus - count.pplane;
        kdtree::KdCost cost = kdtree::CalculateCost(
            parent.boundary, iter->plane, nl, nr, count.pplane);
        best = std::min(best, kdtree::KdSplit{iter->plane, cost});
        nl = nl + count.pplus + count.pplane;
      }
    }
  }
  return best;
}

template <class T>
void append(std::vector<T>& a, const std::vector<T>& b) {
  a.insert(a.end(), b.cbegin(), b.cend());
}

kdtree::KdNode* BuildHelper(unsigned int depth, const kdtree::KdBox& parent) {
  // sizeof(kdtree::KdNode) * node count = 32 * 2^20 = 32 MB
  if (depth >= 20 || parent.triangles.empty()) {
    return new kdtree::KdNode(
        new std::vector<const geometry::Triangle*>(parent.triangles));
  }

  std::set<Event> splits = ListPerfectSplits(parent);
  kdtree::KdSplit split = FindBestSplit(parent, splits);
  if (split.cost.cost > kdtree::LeafCostBound(parent.triangles.size())) {
    return new kdtree::KdNode(
        new std::vector<const geometry::Triangle*>(parent.triangles));
  } else {
    geometry::AabbSplit aabbs = geometry::Split(parent.boundary, split.plane);
    kdtree::IntersectResults triangles =
        kdtree::PartitionTriangles(parent.triangles, split.plane);
    std::vector<const geometry::Triangle*> left_tris(triangles.left);
    std::vector<const geometry::Triangle*> right_tris(triangles.right);
    // Put plane-triangles on side with fewest triangels, or left if both equal.
    if (triangles.left.size() <= triangles.right.size()) {
      append(left_tris, triangles.plane);
    } else {
      // triangles.left.size() > triangles.right.size()
      append(right_tris, triangles.plane);
    }
    kdtree::KdBox left{aabbs.left, left_tris};
    kdtree::KdBox right{aabbs.right, right_tris};
    return new kdtree::KdNode(split.plane, BuildHelper(depth + 1, left),
                              BuildHelper(depth + 1, right));
  }
}

#endif  // KDTREE_SAH_FAST_IMPL_H_
