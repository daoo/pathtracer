#include "kdtree/surface_area_heuristic.h"

#include <glm/glm.hpp>

#include <cassert>
#include <set>
#include <vector>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "geometry/triangle.h"
#include "kdtree/linked.h"
#include "kdtree/util.h"

using geometry::Aabb;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using std::set;
using std::vector;

namespace kdtree {
namespace {
constexpr float COST_TRAVERSE = 0.3f;
constexpr float COST_INTERSECT = 1.0f;
constexpr float EPSILON = 0.00001f;

struct CostSplit {
  Split split;
  float cost;
};

float calculate_cost(const Box& parent, const Split& split) {
  float parent_area = parent.boundary.GetSurfaceArea();
  float left_area = split.left.boundary.GetSurfaceArea();
  float left_count = split.left.triangles.size();
  float right_area = split.right.boundary.GetSurfaceArea();
  float right_count = split.right.triangles.size();
  float intersect =
      (left_area * left_count + right_area * right_count) / parent_area;
  return COST_TRAVERSE + COST_INTERSECT * intersect;
}

CostSplit split(const Box& parent, const Aap& plane) {
  Split split = split_box(parent, plane);
  float cost = calculate_cost(parent, split);
  return CostSplit{split, cost};
}

const CostSplit& get_best(const CostSplit& a, const CostSplit& b) {
  return a.cost <= b.cost ? a : b;
}

void find_perfect_splits(const Aabb& boundary,
                         const Triangle& triangle,
                         Axis axis,
                         set<Aap>* splits) {
  float boundary_min = boundary.GetMin()[axis];
  float boundary_max = boundary.GetMax()[axis];
  float min = triangle.GetMin()[axis] - EPSILON;
  float max = triangle.GetMax()[axis] + EPSILON;
  splits->emplace(axis, glm::clamp(min, boundary_min, boundary_max));
  splits->emplace(axis, glm::clamp(max, boundary_min, boundary_max));
}

void find_perfect_splits(const Aabb& boundary,
                         const Triangle& triangle,
                         set<Aap>* splits) {
  find_perfect_splits(boundary, triangle, geometry::X, splits);
  find_perfect_splits(boundary, triangle, geometry::Y, splits);
  find_perfect_splits(boundary, triangle, geometry::Z, splits);
}

set<Aap> find_perfect_splits(const Box& box) {
  set<Aap> splits;
  for (const Triangle* triangle : box.triangles) {
    find_perfect_splits(box.boundary, *triangle, &splits);
  }
  return splits;
}

CostSplit find_best(const Box& box, const set<Aap>& splits) {
  assert(splits.size() > 0);
  auto it = splits.begin();
  CostSplit best = split(box, *it);
  ++it;
  while (it != splits.end()) {
    best = get_best(best, split(box, *it));
    ++it;
  }
  return best;
}

KdNodeLinked* go(unsigned int depth, const Box& parent) {
  // sizeof(KdNodeLinked) * node count = 32 * 2^20 = 32 MB
  if (depth >= 20 || parent.triangles.empty()) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  }

  set<Aap> splits = find_perfect_splits(parent);
  CostSplit split = find_best(parent, splits);
  float leaf_cost = COST_INTERSECT * parent.triangles.size();
  if (split.cost > leaf_cost) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  } else {
    return new KdNodeLinked(split.split.plane, go(depth + 1, split.split.left),
                            go(depth + 1, split.split.right));
  }
}
}  // namespace

KdTreeLinked build_tree_sah(const vector<Triangle>& triangles) {
  vector<const Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  return {go(0, Box{find_bounding(triangles), ptrs})};
}
}  // namespace kdtree
