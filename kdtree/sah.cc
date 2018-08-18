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
#include "kdtree/linked.h"
#include "kdtree/sah_common.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

namespace {

struct KdCostSplit {
  kdtree::KdSplit split;
  float cost;

  bool operator<(const KdCostSplit& other) const { return cost < other.cost; }
};

float CalculateCost(const geometry::Aabb& parent,
                    const kdtree::KdBox& left,
                    const kdtree::KdBox& right) {
  float parent_area = parent.GetSurfaceArea();
  float left_area = left.boundary.GetSurfaceArea();
  float right_area = right.boundary.GetSurfaceArea();
  size_t left_count = left.triangles.size();
  size_t right_count = right.triangles.size();
  return kdtree::CalculateCost(parent_area, left_area, right_area, left_count,
                               right_count);
}

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

KdCostSplit SplitWithCost(const kdtree::KdBox& parent,
                          const geometry::Aap& plane) {
  // TODO: calculate which side
  kdtree::KdSplit split = kdtree::Split(parent, plane, kdtree::LEFT);
  float cost = CalculateCost(parent.boundary, split.left, split.right);
  return {split, cost};
}

KdCostSplit FindBestSplit(const kdtree::KdBox& parent,
                          const std::set<geometry::Aap>& splits) {
  assert(splits.size() > 0);
  auto it = splits.begin();
  KdCostSplit best = SplitWithCost(parent, *it);
  ++it;
  while (it != splits.end()) {
    best = std::min(best, SplitWithCost(parent, *it));
    ++it;
  }
  return best;
}

kdtree::KdNodeLinked* BuildHelper(unsigned int depth,
                                  const kdtree::KdBox& parent) {
  // sizeof(kdtree::KdNodeLinked) * node count = 32 * 2^20 = 32 MB
  if (depth >= 20 || parent.triangles.empty()) {
    return new kdtree::KdNodeLinked(
        new std::vector<const geometry::Triangle*>(parent.triangles));
  }

  std::set<geometry::Aap> splits = ListPerfectSplits(parent);
  KdCostSplit split = FindBestSplit(parent, splits);
  if (split.cost > kdtree::LeafCostBound(parent.triangles.size())) {
    return new kdtree::KdNodeLinked(
        new std::vector<const geometry::Triangle*>(parent.triangles));
  } else {
    return new kdtree::KdNodeLinked(split.split.plane,
                                    BuildHelper(depth + 1, split.split.left),
                                    BuildHelper(depth + 1, split.split.right));
  }
}

}  // namespace

namespace kdtree {
kdtree::KdTreeLinked build(const std::vector<geometry::Triangle>& triangles) {
  std::vector<const geometry::Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const geometry::Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  return kdtree::KdTreeLinked(
      BuildHelper(0, KdBox{geometry::find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
