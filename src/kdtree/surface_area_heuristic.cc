#include "kdtree/surface_area_heuristic.h"

#include <glm/glm.hpp>

#include <cassert>
#include <cstddef>
#include <vector>

#include "geometry/aabb.h"
#include "geometry/bounding.h"
#include "geometry/triangle.h"
#include "kdtree/linked.h"
#include "kdtree/util.h"

using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
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

const CostSplit& get_best(const CostSplit& a,
                          const CostSplit& b,
                          const CostSplit& c) {
  return get_best(a, get_best(b, c));
}

CostSplit find_best(const Box& parent, const Triangle& triangle, Axis axis) {
  float min = triangle.GetMin()[axis] - EPSILON;
  float max = triangle.GetMax()[axis] + EPSILON;
  return get_best(split(parent, {axis, min}), split(parent, {axis, max}));
}

CostSplit find_best(const Box& parent, const Triangle& triangle) {
  return get_best(find_best(parent, triangle, geometry::X),
                  find_best(parent, triangle, geometry::Y),
                  find_best(parent, triangle, geometry::Z));
}

CostSplit find_best(const Box& parent) {
  assert(parent.triangles.size() > 0);
  CostSplit best = find_best(parent, *parent.triangles[0]);
  for (size_t i = 1; i < parent.triangles.size(); ++i) {
    best = get_best(best, find_best(parent, *parent.triangles[i]));
  }
  return best;
}

KdNodeLinked* go(unsigned int depth, const Box& parent) {
  // sizeof(KdNodeLinked) * node count = 32 * 2^20 = 32 MB
  if (depth >= 20 || parent.triangles.empty()) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  }

  CostSplit split = find_best(parent);
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

  return {go(0, Box{find_bounding(triangles), ptrs}), triangles};
}
}  // namespace kdtree
