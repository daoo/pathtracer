#include "kdtree/sah.h"

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include <cassert>

#include <algorithm>
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

using geometry::Aabb;
using geometry::AabbSplit;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using glm::vec3;
using kdtree::KdBox;
using kdtree::KdNodeLinked;
using kdtree::KdSplit;
using std::set;
using std::vector;

namespace {

struct KdCostSplit {
  KdSplit split;
  float cost;

  bool operator<(const KdCostSplit& other) const { return cost < other.cost; }
};

float CalculateCost(const Aabb& parent, const KdBox& left, const KdBox& right) {
  float parent_area = parent.GetSurfaceArea();
  float left_area = left.boundary.GetSurfaceArea();
  float right_area = right.boundary.GetSurfaceArea();
  size_t left_count = left.triangles.size();
  size_t right_count = right.triangles.size();
  return kdtree::CalculateCost(parent_area, left_area, right_area, left_count,
                               right_count);
}

void ListPerfectSplits(const Aabb& boundary,
                       const Triangle& triangle,
                       Axis axis,
                       set<Aap>* splits) {
  float boundary_min = boundary.GetMin()[axis];
  float boundary_max = boundary.GetMax()[axis];
  float triangle_min = triangle.GetMin()[axis] - glm::epsilon<float>();
  float triangle_max = triangle.GetMax()[axis] + glm::epsilon<float>();
  float clamped_min = glm::clamp(triangle_min, boundary_min, boundary_max);
  float clamped_max = glm::clamp(triangle_max, boundary_min, boundary_max);
  splits->emplace(axis, clamped_min);
  splits->emplace(axis, clamped_max);
}

void ListPerfectSplits(const Aabb& boundary,
                       const Triangle& triangle,
                       set<Aap>* splits) {
  ListPerfectSplits(boundary, triangle, geometry::X, splits);
  ListPerfectSplits(boundary, triangle, geometry::Y, splits);
  ListPerfectSplits(boundary, triangle, geometry::Z, splits);
}

set<Aap> ListPerfectSplits(const KdBox& box) {
  set<Aap> splits;
  for (const Triangle* triangle : box.triangles) {
    ListPerfectSplits(box.boundary, *triangle, &splits);
  }
  return splits;
}

KdCostSplit SplitWithCost(const KdBox& parent, const Aap& plane) {
  KdSplit split = kdtree::Split(parent, plane);
  float cost = CalculateCost(parent.boundary, split.left, split.right);
  return {split, cost};
}

KdCostSplit FindBestSplit(const KdBox& box, const set<Aap>& splits) {
  assert(splits.size() > 0);
  auto it = splits.begin();
  KdCostSplit best = SplitWithCost(box, *it);
  ++it;
  while (it != splits.end()) {
    best = std::min(best, SplitWithCost(box, *it));
    ++it;
  }
  return best;
}

KdNodeLinked* BuildHelper(unsigned int depth, const KdBox& parent) {
  // sizeof(KdNodeLinked) * node count = 32 * 2^20 = 32 MB
  if (depth >= 20 || parent.triangles.empty()) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  }

  KdCostSplit split = FindBestSplit(parent, ListPerfectSplits(parent));
  if (split.cost > kdtree::LeafCostBound(parent.triangles.size())) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  } else {
    return new KdNodeLinked(split.split.plane,
                            BuildHelper(depth + 1, split.split.left),
                            BuildHelper(depth + 1, split.split.right));
  }
}

}  // namespace

namespace kdtree {
KdTreeLinked build(const vector<Triangle>& triangles) {
  vector<const Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  return KdTreeLinked(
      BuildHelper(0, KdBox{geometry::find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
