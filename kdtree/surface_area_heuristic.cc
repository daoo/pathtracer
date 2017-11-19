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
#include "geometry/tribox.h"
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

struct IntersectResults {
  vector<const Triangle*> left;
  vector<const Triangle*> right;
};

IntersectResults intersect_test(const vector<const Triangle*>& triangles,
                                const Aabb& left_aabb,
                                const Aabb& right_aabb) {
  IntersectResults results;
  results.left.reserve(triangles.size());
  results.right.reserve(triangles.size());
  for (const Triangle* triangle : triangles) {
    bool in_left =
        tri_box_overlap(left_aabb, triangle->v0, triangle->v1, triangle->v2);
    bool in_right =
        tri_box_overlap(right_aabb, triangle->v0, triangle->v1, triangle->v2);
    assert(in_left || in_right);
    if (in_left) results.left.emplace_back(triangle);
    if (in_right) results.right.emplace_back(triangle);
  }

  results.left.shrink_to_fit();
  results.right.shrink_to_fit();
  return results;
}

struct Box {
  geometry::Aabb boundary;
  std::vector<const geometry::Triangle*> triangles;
};

constexpr float COST_TRAVERSE = 0.3f;
constexpr float COST_INTERSECT = 1.0f;

float calculate_cost(const Aabb& parent, const Box& left, const Box& right) {
  float parent_area = parent.GetSurfaceArea();
  float left_area = left.boundary.GetSurfaceArea();
  float left_count = left.triangles.size();
  float right_area = right.boundary.GetSurfaceArea();
  float right_count = right.triangles.size();
  float intersect =
      (left_area * left_count + right_area * right_count) / parent_area;
  return COST_TRAVERSE + COST_INTERSECT * intersect;
}

void list_perfect_splits(const Aabb& boundary,
                         const Triangle& triangle,
                         Axis axis,
                         set<Aap>* splits) {
  float boundary_min = boundary.GetMin()[axis];
  float boundary_max = boundary.GetMax()[axis];
  float min = triangle.GetMin()[axis] - glm::epsilon<float>();
  float max = triangle.GetMax()[axis] + glm::epsilon<float>();
  splits->emplace(axis, glm::clamp(min, boundary_min, boundary_max));
  splits->emplace(axis, glm::clamp(max, boundary_min, boundary_max));
}

void list_perfect_splits(const Aabb& boundary,
                         const Triangle& triangle,
                         set<Aap>* splits) {
  list_perfect_splits(boundary, triangle, geometry::X, splits);
  list_perfect_splits(boundary, triangle, geometry::Y, splits);
  list_perfect_splits(boundary, triangle, geometry::Z, splits);
}

set<Aap> list_perfect_splits(const Box& box) {
  set<Aap> splits;
  for (const Triangle* triangle : box.triangles) {
    list_perfect_splits(box.boundary, *triangle, &splits);
  }
  return splits;
}

struct Split {
  geometry::Aap plane;
  Box left, right;
  float cost;

  bool operator<(const Split& other) const { return cost < other.cost; }
};

Split split_box(const Box& parent, const Aap& plane) {
  AabbSplit aabbs = split(parent.boundary, plane);
  glm::vec3 delta(0, 0, 0);
  delta[plane.GetAxis()] = glm::epsilon<float>();
  Aabb left_aabb = aabbs.left.Translate(-delta).Enlarge(delta);
  Aabb right_aabb = aabbs.right.Translate(delta).Enlarge(delta);
  IntersectResults triangles =
      intersect_test(parent.triangles, left_aabb, right_aabb);
  Box left{left_aabb, triangles.left};
  Box right{right_aabb, triangles.right};
  float cost = calculate_cost(parent.boundary, left, right);
  return Split{plane, left, right, cost};
}

Split find_best_split(const Box& box, const set<Aap>& splits) {
  assert(splits.size() > 0);
  auto it = splits.begin();
  Split best = split_box(box, *it);
  ++it;
  while (it != splits.end()) {
    best = std::min(best, split_box(box, *it));
    ++it;
  }
  return best;
}

KdNodeLinked* go(unsigned int depth, const Box& parent) {
  // sizeof(KdNodeLinked) * node count = 32 * 2^20 = 32 MB
  if (depth >= 20 || parent.triangles.empty()) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  }

  Split split = find_best_split(parent, list_perfect_splits(parent));
  float leaf_cost = COST_INTERSECT * parent.triangles.size();
  if (split.cost > leaf_cost) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  } else {
    return new KdNodeLinked(split.plane, go(depth + 1, split.left),
                            go(depth + 1, split.right));
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

  return KdTreeLinked(go(0, Box{find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
