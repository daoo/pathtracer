#ifndef UTIL_HPP_26AQ5WXE
#define UTIL_HPP_26AQ5WXE

#include "geometry/aabb.hpp"
#include "geometry/tribox.hpp"
#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>
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

inline void split_aabb(const geometry::Aabb& box,
                Axis axis,
                float split,
                geometry::Aabb& left,
                geometry::Aabb& right) {
  left = box;
  right = box;

  float split_clamped = glm::clamp(split, box.center[axis] - box.half[axis],
                                   box.center[axis] + box.half[axis]);

  float min = box.center[axis] - box.half[axis];
  float max = box.center[axis] + box.half[axis];

  float lh = (split_clamped - min) / 2.0f + glm::epsilon<float>();
  float rh = (max - split_clamped) / 2.0f + glm::epsilon<float>();

  left.half[axis] = lh;
  left.center[axis] = split_clamped - lh;

  right.half[axis] = rh;
  right.center[axis] = split_clamped + rh;
}

inline void intersect_test(const geometry::Aabb& left_box,
                    const geometry::Aabb& right_box,
                    const std::vector<const geometry::Triangle*>& triangles,
                    std::vector<const geometry::Triangle*>& left_triangles,
                    std::vector<const geometry::Triangle*>& right_triangles) {
  left_triangles.reserve(triangles.size());
  right_triangles.reserve(triangles.size());
  for (const geometry::Triangle* tri : triangles) {
    if (tri_box_overlap(left_box, tri->v0, tri->v1, tri->v2)) {
      left_triangles.push_back(tri);
    }

    if (tri_box_overlap(right_box, tri->v0, tri->v1, tri->v2)) {
      right_triangles.push_back(tri);
    }
  }

  assert(left_triangles.size() + right_triangles.size() >= triangles.size());

  left_triangles.shrink_to_fit();
  right_triangles.shrink_to_fit();
}
}

#endif /* end of include guard: UTIL_HPP_26AQ5WXE */
