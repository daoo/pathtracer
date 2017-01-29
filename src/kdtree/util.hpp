#ifndef UTIL_HPP_26AQ5WXE
#define UTIL_HPP_26AQ5WXE

#include "geometry/aabb.hpp"
#include "geometry/tribox.hpp"
#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>
#include <tuple>
#include <vector>

namespace kdtree {
enum Axis { X = 0, Y = 1, Z = 2 };

// A look up table have been empirically proven to be the fastest way to
// calculate the next axis, compared to using modulo addition and bit hacks.
constexpr Axis NEXT[] = {Y, Z, X};
constexpr inline Axis next_axis(Axis axis) {
  return NEXT[axis];
}

static_assert(next_axis(X) == Y, "incorrect next");
static_assert(next_axis(Y) == Z, "incorrect next");
static_assert(next_axis(Z) == X, "incorrect next");

struct Box {
  geometry::Aabb boundary;
  std::vector<const geometry::Triangle*> triangles;
};

struct Split {
  Axis axis;
  float distance;
  Box left, right;
};

inline std::tuple<geometry::Aabb, geometry::Aabb>
split_aabb(const geometry::Aabb& parent, Axis axis, float split) {
  geometry::Aabb left(parent), right(parent);

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

inline std::tuple<std::vector<const geometry::Triangle*>,
                  std::vector<const geometry::Triangle*>>
intersect_test(const std::vector<const geometry::Triangle*>& triangles,
               const geometry::Aabb& left_aabb,
               const geometry::Aabb& right_aabb) {
  std::vector<const geometry::Triangle*> left_triangles;
  std::vector<const geometry::Triangle*> right_triangles;
  for (const geometry::Triangle* tri : triangles) {
    if (tri_box_overlap(left_aabb, tri->v0, tri->v1, tri->v2)) {
      left_triangles.push_back(tri);
    }

    if (tri_box_overlap(right_aabb, tri->v0, tri->v1, tri->v2)) {
      right_triangles.push_back(tri);
    }
  }

  assert(left_triangles.size() + right_triangles.size() >= triangles.size());

  return std::make_tuple(left_triangles, right_triangles);
}

inline Split split_box(const Box& parent, Axis axis, float distance) {
  auto aabbs = split_aabb(parent.boundary, axis, distance);
  auto triangles =
      intersect_test(parent.triangles, std::get<0>(aabbs), std::get<1>(aabbs));
  return Split{axis, distance, Box{std::get<0>(aabbs), std::get<0>(triangles)},
               Box{std::get<1>(aabbs), std::get<1>(triangles)}};
}
}

#endif /* end of include guard: UTIL_HPP_26AQ5WXE */
