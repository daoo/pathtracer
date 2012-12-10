#ifndef UTIL_HPP_26AQ5WXE
#define UTIL_HPP_26AQ5WXE

#include "math/aabb.hpp"
#include "pathtracer/triangle.hpp"
#include <glm/glm.hpp>

namespace kdtree {
  enum Axis {
    X = 0, Y = 1, Z = 2
  };

  namespace helpers {
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

    inline void triangleExtremes(const Triangle& tri, Axis axis, float& min, float& max) {
      float a = swizzle(tri.v0, axis);
      float b = swizzle(tri.v1, axis);
      float c = swizzle(tri.v2, axis);

      min = glm::min(glm::min(a, b), c);
      max = glm::max(glm::max(a, b), c);
    }

    inline void aabbFromSplit(const math::Aabb& box, Axis axis, float split, math::Aabb& left, math::Aabb& right) {
      left  = box;
      right = box;

      if (axis == X) {
        float lh = (split - (box.center.x - box.half.x)) / 2.0f;
        left.half.x   = lh;
        left.center.x = split - lh;

        float rh = ((box.center.x + box.half.x) - split) / 2.0f;
        right.half.x   = rh;
        right.center.x = split + rh;
      } else if (axis == Y) {
        float lh = (split - (box.center.y - box.half.y)) / 2.0f;
        left.half.y   = lh;
        left.center.y = split - lh;

        float rh = ((box.center.y + box.half.y) - split) / 2.0f;
        right.half.y   = rh;
        right.center.y = split + rh;
      } else if (axis == Z) {
        float lh = (split - (box.center.z - box.half.z)) / 2.0f;
        left.half.z   = lh;
        left.center.z = split - lh;

        float rh = ((box.center.z + box.half.z) - split) / 2.0f;
        right.half.z   = rh;
        right.center.z = split + rh;
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
}

#endif /* end of include guard: UTIL_HPP_26AQ5WXE */
