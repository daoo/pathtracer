#ifndef UTIL_HPP_26AQ5WXE
#define UTIL_HPP_26AQ5WXE

#include "math/aabb.hpp"
#include "pathtracer/triangle.hpp"
#include <glm/glm.hpp>

namespace kdtree {
  enum Axis {
    X = 0, Y = 1, Z = 2
  };

  template <typename T>
  inline void order(float t, T& first, T& second) {
    if (t < 0) {
      std::swap(first, second);
    }
  }

  inline float middle(float a, float b) {
    assert(a < b);
    return a + (b - a) / 2.0f;
  }

  inline float swizzle(const glm::vec3& v, Axis c) {
    switch (c) {
      case X: return v.x;
      case Y: return v.y;
      case Z: return v.z;
    }
  }

  inline void triangleExtremes(const Triangle& tri, Axis axis,
      float& min, float& max) {
    float a = swizzle(tri.v0, axis);
    float b = swizzle(tri.v1, axis);
    float c = swizzle(tri.v2, axis);

    min = glm::min(glm::min(a, b), c);
    max = glm::max(glm::max(a, b), c);
  }

  inline void aabbFromSplit(const math::Aabb& box,
      Axis axis, float split, math::Aabb& left, math::Aabb& right) {
    constexpr float EPSILON = 0.000001f;

    left  = box;
    right = box;

    float splitClamped = glm::clamp(
        split,
        swizzle(box.center, axis) - swizzle(box.half, axis),
        swizzle(box.center, axis) + swizzle(box.half, axis));

    if (axis == X) {
      float min = box.center.x - box.half.x;
      float max = box.center.x + box.half.x;

      float lh = (splitClamped - min) / 2.0f + EPSILON;
      left.half.x   = lh;
      left.center.x = splitClamped - lh;

      float rh = (max - splitClamped) / 2.0f + EPSILON;
      right.half.x   = rh;
      right.center.x = splitClamped + rh;
    } else if (axis == Y) {
      float min = box.center.y - box.half.y;
      float max = box.center.y + box.half.y;

      float lh = (splitClamped - min) / 2.0f + EPSILON;
      left.half.y   = lh;
      left.center.y = splitClamped - lh;

      float rh = (max - splitClamped) / 2.0f + EPSILON;
      right.half.y   = rh;
      right.center.y = splitClamped + rh;
    } else if (axis == Z) {
      float min = box.center.z - box.half.z;
      float max = box.center.z + box.half.z;

      float lh = (splitClamped - min) / 2.0f + EPSILON;
      left.half.z   = lh;
      left.center.z = splitClamped - lh;

      float rh = (max - splitClamped) / 2.0f + EPSILON;
      right.half.z   = rh;
      right.center.z = splitClamped + rh;
    }
  }

  inline math::Aabb findBounding(const std::vector<Triangle>& triangles) {
    glm::vec3 min, max;

    for (const Triangle& tri : triangles) {
      min = glm::min(min, tri.v0);
      min = glm::min(min, tri.v1);
      min = glm::min(min, tri.v2);

      max = glm::max(max, tri.v0);
      max = glm::max(max, tri.v1);
      max = glm::max(max, tri.v2);
    }

    glm::vec3 half = (max - min) / 2.0f;
    return { min + half, half };
  }
}

#endif /* end of include guard: UTIL_HPP_26AQ5WXE */
