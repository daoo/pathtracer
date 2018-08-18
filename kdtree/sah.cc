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

void ListPerfectSplits(const geometry::Aabb& boundary,
                       const geometry::Triangle& triangle,
                       geometry::Axis axis,
                       std::set<geometry::Aap>* splits) {
  float boundary_min = boundary.GetMin()[axis];
  float boundary_max = boundary.GetMax()[axis];
  float triangle_min = triangle.GetMin()[axis] - glm::epsilon<float>();
  float triangle_max = triangle.GetMax()[axis] + glm::epsilon<float>();
  float clamped_min = glm::clamp(triangle_min, boundary_min, boundary_max);
  float clamped_max = glm::clamp(triangle_max, boundary_min, boundary_max);
  splits->emplace(axis, clamped_min);
  splits->emplace(axis, clamped_max);
}

void ListPerfectSplits(const geometry::Aabb& boundary,
                       const geometry::Triangle& triangle,
                       std::set<geometry::Aap>* splits) {
  ListPerfectSplits(boundary, triangle, geometry::X, splits);
  ListPerfectSplits(boundary, triangle, geometry::Y, splits);
  ListPerfectSplits(boundary, triangle, geometry::Z, splits);
}

std::set<geometry::Aap> ListPerfectSplits(const kdtree::KdBox& parent) {
  std::set<geometry::Aap> splits;
  for (const geometry::Triangle* triangle : parent.triangles) {
    ListPerfectSplits(parent.boundary, *triangle, &splits);
  }
  return splits;
}

kdtree::KdSplit SplitWithCost(const kdtree::KdBox& parent,
                              const geometry::Aap& plane) {
  kdtree::IntersectResults triangles =
      kdtree::PartitionTriangles(parent.triangles, plane);
  kdtree::KdCost cost =
      kdtree::CalculateCost(parent.boundary, plane, triangles.left.size(),
                            triangles.right.size(), triangles.plane.size());
  return kdtree::KdSplit{plane, cost};
}

kdtree::KdSplit FindBestSplit(const kdtree::KdBox& parent,
                              const std::set<geometry::Aap>& splits) {
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

  std::set<geometry::Aap> splits = ListPerfectSplits(parent);
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
