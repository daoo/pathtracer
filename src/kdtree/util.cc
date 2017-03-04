#include "kdtree/util.h"

#include <cassert>
#include <glm/gtc/constants.hpp>
#include <tuple>

#include "geometry/aabb.h"
#include "geometry/triangle.h"
#include "geometry/tribox.h"

using geometry::Aabb;
using geometry::Triangle;
using std::tuple;
using std::vector;

namespace kdtree {
namespace {
tuple<Aabb, Aabb> split_aabb(const Aabb& parent, Axis axis, float split) {
  Aabb left(parent), right(parent);

  float min = parent.center[axis] - parent.half[axis];
  float max = parent.center[axis] + parent.half[axis];
  float split_clamped = glm::clamp(split, min, max);
  float lh = (split_clamped - min) / 2.0f + glm::epsilon<float>();
  float rh = (max - split_clamped) / 2.0f + glm::epsilon<float>();

  left.half[axis] = lh;
  left.center[axis] = split_clamped - lh;

  right.half[axis] = rh;
  right.center[axis] = split_clamped + rh;

  return std::make_tuple(left, right);
}

tuple<vector<const Triangle*>, vector<const Triangle*>> intersect_test(
    const vector<const Triangle*>& triangles,
    const Aabb& left_aabb,
    const Aabb& right_aabb) {
  vector<const Triangle*> left_triangles;
  vector<const Triangle*> right_triangles;
  for (const Triangle* tri : triangles) {
    if (tri_box_overlap(left_aabb, tri->v0, tri->v1, tri->v2)) {
      left_triangles.emplace_back(tri);
    }

    if (tri_box_overlap(right_aabb, tri->v0, tri->v1, tri->v2)) {
      right_triangles.emplace_back(tri);
    }
  }

  assert(left_triangles.size() + right_triangles.size() >= triangles.size());

  return std::make_tuple(left_triangles, right_triangles);
}
}  // namespace

Split split_box(const Box& parent, Axis axis, float distance) {
  auto aabbs = split_aabb(parent.boundary, axis, distance);
  auto triangles =
      intersect_test(parent.triangles, std::get<0>(aabbs), std::get<1>(aabbs));
  return Split{axis, distance, Box{std::get<0>(aabbs), std::get<0>(triangles)},
               Box{std::get<1>(aabbs), std::get<1>(triangles)}};
}
}  // namespace kdtree
