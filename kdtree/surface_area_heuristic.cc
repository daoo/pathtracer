#include "kdtree/build.h"

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
#include "kdtree/intersect.h"
#include "kdtree/linked.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

using geometry::Aabb;
using geometry::AabbSplit;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using glm::vec3;
using kdtree::KdNodeLinked;
using std::set;
using std::vector;

namespace {

struct KdBox {
  Aabb boundary;
  std::vector<const Triangle*> triangles;
};

struct KdSplit {
  Aap plane;
  KdBox left, right;
  float cost;

  bool operator<(const KdSplit& other) const { return cost < other.cost; }
};

constexpr float COST_TRAVERSE = 0.01f;
constexpr float COST_INTERSECT = 1.0f;
constexpr float COST_EMPTY = 0.8f;

float CalculateCost(float parent_area,
                    float left_area,
                    float right_area,
                    size_t left_count,
                    size_t right_count) {
  float factor = left_count == 0 || right_count == 0 ? COST_EMPTY : 1.0f;
  float intersect =
      (left_area * left_count + right_area * right_count) / parent_area;
  return factor * COST_TRAVERSE + COST_INTERSECT * intersect;
}

float CalculateCost(const Aabb& parent, const KdBox& left, const KdBox& right) {
  float parent_area = parent.GetSurfaceArea();
  float left_area = left.boundary.GetSurfaceArea();
  float right_area = right.boundary.GetSurfaceArea();
  size_t left_count = left.triangles.size();
  size_t right_count = right.triangles.size();
  return CalculateCost(parent_area, left_area, right_area, left_count,
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

KdSplit Split(const KdBox& parent, const Aap& plane) {
  AabbSplit aabbs = geometry::Split(parent.boundary, plane);
  kdtree::IntersectResults triangles =
      kdtree::PartitionTriangles(parent.triangles, plane);
  KdBox left{aabbs.left, triangles.left};
  KdBox right{aabbs.right, triangles.right};
  float cost = CalculateCost(parent.boundary, left, right);
  return KdSplit{plane, left, right, cost};
}

KdSplit FindBestSplit(const KdBox& box, const set<Aap>& splits) {
  assert(splits.size() > 0);
  auto it = splits.begin();
  KdSplit best = Split(box, *it);
  ++it;
  while (it != splits.end()) {
    best = std::min(best, Split(box, *it));
    ++it;
  }
  return best;
}

KdNodeLinked* BuildHelper(unsigned int depth, const KdBox& parent) {
  // sizeof(KdNodeLinked) * node count = 32 * 2^20 = 32 MB
  if (depth >= 20 || parent.triangles.empty()) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  }

  KdSplit split = FindBestSplit(parent, ListPerfectSplits(parent));
  float leaf_cost = COST_INTERSECT * parent.triangles.size();
  if (split.cost > leaf_cost) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  } else {
    return new KdNodeLinked(split.plane, BuildHelper(depth + 1, split.left),
                            BuildHelper(depth + 1, split.right));
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
