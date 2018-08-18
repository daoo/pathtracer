#include "kdtree/sah.h"

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

namespace {

kdtree::KdSplit SplitWithCost(const kdtree::KdBox& parent,
                              const kdtree::Event& event) {
  kdtree::IntersectResults triangles =
      kdtree::PartitionTriangles(parent.triangles, event.plane);
  kdtree::KdCost cost =
      kdtree::CalculateCost(parent.boundary, event.plane, triangles.left.size(),
                            triangles.right.size(), triangles.plane.size());
  return kdtree::KdSplit{event.plane, cost};
}

kdtree::KdSplit FindBestSplit(const kdtree::KdBox& parent,
                              const std::set<kdtree::Event>& splits) {
  assert(splits.size() > 0);
  auto it = splits.begin();
  kdtree::KdSplit best = SplitWithCost(parent, *it);
  ++it;
  while (it != splits.end()) {
    best = std::min(best, SplitWithCost(parent, *it));
    ++it;
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

}  // namespace

namespace kdtree {
kdtree::KdTree build(const std::vector<geometry::Triangle>& triangles) {
  std::vector<const geometry::Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const geometry::Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  return kdtree::KdTree(
      BuildHelper(0, KdBox{geometry::find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
