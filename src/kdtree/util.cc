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
  float min = parent.GetMin()[axis];
  float max = parent.GetMax()[axis];
  float split_clamped = glm::clamp(split, min, max);
  float lh = (split_clamped - min) / 2.0f + glm::epsilon<float>();
  float rh = (max - split_clamped) / 2.0f + glm::epsilon<float>();

  glm::vec3 left_center(parent.GetCenter()), left_half(parent.GetHalf());
  left_center[axis] = split_clamped - lh;
  left_half[axis] = lh;

  glm::vec3 right_center(parent.GetCenter()), right_half(parent.GetHalf());
  right_center[axis] = split_clamped + rh;
  right_half[axis] = rh;

  return std::make_tuple(Aabb(left_center, left_half),
                         Aabb(right_center, right_half));
}

tuple<vector<const Triangle*>, vector<const Triangle*>> intersect_test(
    const vector<const Triangle*>& triangles,
    const Aabb& left_aabb,
    const Aabb& right_aabb) {
  vector<const Triangle*> left_triangles;
  vector<const Triangle*> right_triangles;
  left_triangles.reserve(triangles.size());
  right_triangles.reserve(triangles.size());
  for (const Triangle* tri : triangles) {
    if (tri_box_overlap(left_aabb, tri->v0, tri->v1, tri->v2)) {
      left_triangles.emplace_back(tri);
    }

    if (tri_box_overlap(right_aabb, tri->v0, tri->v1, tri->v2)) {
      right_triangles.emplace_back(tri);
    }
  }

  assert(left_triangles.size() + right_triangles.size() >= triangles.size());

  left_triangles.shrink_to_fit();
  right_triangles.shrink_to_fit();
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
