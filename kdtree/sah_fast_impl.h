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

struct EventCount {
  size_t pminus, pplus, pplane;
};

EventCount CountEvents(std::set<kdtree::Event>::const_iterator begin,
                       std::set<kdtree::Event>::const_iterator end) {
  assert(begin != end);
  size_t pminus = 0;
  size_t pplus = 0;
  size_t pplane = 0;
  float distance = begin->plane.GetDistance();
  geometry::Axis axis = begin->plane.GetAxis();
  auto iter = begin;
  while (iter != end && iter->plane.GetDistance() == distance &&
         iter->plane.GetAxis() == axis && iter->type == kdtree::END) {
    pminus += 1;
    ++iter;
  }
  while (iter != end && iter->plane.GetDistance() == distance &&
         iter->plane.GetAxis() == axis && iter->type == kdtree::PLANAR) {
    pplane += 1;
    ++iter;
  }
  while (iter != end && iter->plane.GetDistance() == distance &&
         iter->plane.GetAxis() == axis && iter->type == kdtree::START) {
    pplus += 1;
    ++iter;
  }
  return EventCount{pminus, pplus, pplane};
}

kdtree::KdSplit FindBestSplit(const kdtree::KdBox& parent,
                              const std::set<kdtree::Event>& splits) {
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

  std::set<kdtree::Event> splits = ListPerfectSplits(parent);
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
