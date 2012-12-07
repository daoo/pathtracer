#ifndef UTIL_HPP_26AQ5WXE
#define UTIL_HPP_26AQ5WXE

#include "pathtracer/math/aabb.hpp"
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
