#ifndef UTIL_HPP_26AQ5WXE
#define UTIL_HPP_26AQ5WXE

#include "math/aabb.hpp"
#include "triangle.hpp"
#include <cassert>
#include <glm/glm.hpp>

namespace kdtree {
  enum Axis {
    X = 0, Y = 1, Z = 2
  };

  namespace helpers {
    bool containsLeft(const Triangle& tri, float d, Axis axis);
    bool containsRight(const Triangle& tri, float d, Axis axis);

    float middle(float, float);
    float swizzle(const glm::vec3&, Axis);
    float& swizzle(glm::vec3&, Axis);

    math::Aabb findBounding(const std::vector<Triangle>& triangles);
    math::Aabb splitAabb(const math::Aabb&, float, Axis);

    template <typename T>
    inline void order(float t, T& first, T& second) {
      if (t < 0) {
        std::swap(first, second);
      }
    }

    template <typename T>
    inline void order(float t, const T* a, const T* b, const T*& first, const T*& second) {
      if (t > 0) {
        first  = a;
        second = b;
      } else {
        first  = b;
        second = a;
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

    inline float& swizzle(glm::vec3& v, Axis c) {
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

      return { min, max };
    }

    inline bool containsLeft(const Triangle& tri, float d, Axis axis) {
      return swizzle(tri.v0, axis) <= d
          || swizzle(tri.v1, axis) <= d
          || swizzle(tri.v2, axis) <= d;
    }

    inline bool containsRight(const Triangle& tri, float d, Axis axis) {
      return swizzle(tri.v0, axis) >= d
          || swizzle(tri.v1, axis) >= d
          || swizzle(tri.v2, axis) >= d;
    }
  }
}

#endif /* end of include guard: UTIL_HPP_26AQ5WXE */
