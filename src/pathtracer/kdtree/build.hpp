#ifndef BUILD_HPP_BTVASI2S
#define BUILD_HPP_BTVASI2S

#include "pathtracer/kdtree/util.hpp"
#include "pathtracer/math/aabb.hpp"
#include "pathtracer/math/tribox.hpp"
#include "pathtracer/triangle.hpp"

#include <array>
#include <vector>

namespace kdtree {
  template <typename Iter>
  void buildTree(Iter iter, const math::Aabb& bounding, const std::vector<Triangle>& triangles) {
    constexpr float epsilon = 0.0000001f;
    const glm::vec3 vec_epsilon(epsilon);

    if (iter.depth() >= 13 || triangles.size() <= 10) {
      iter.leaf(triangles);
    } else {
      glm::vec3 new_size = bounding.half;
      glm::vec3 offset(0, 0, 0);
      float d;

      if (iter.axis() == X) {
        d = bounding.center.x;

        new_size.x /= 2.0f;
        offset.x = new_size.x;
      } else if (iter.axis() == Y) {
        d = bounding.center.y;

        new_size.y /= 2.0f;
        offset.y = new_size.y;
      } else if (iter.axis() == Z) {
        d = bounding.center.z;

        new_size.z /= 2.0f;
        offset.z = new_size.z;
      }

      math::Aabb left_bounding({ bounding.center - offset, new_size + epsilon });
      math::Aabb right_bounding({ bounding.center + offset, new_size + epsilon });

      std::vector<Triangle> left_triangles, right_triangles;
      for (const Triangle& tri : triangles) {
        if (triBoxOverlap(left_bounding, tri.v0, tri.v1, tri.v2)) {
          left_triangles.push_back(tri);
        }

        if (triBoxOverlap(right_bounding, tri.v0, tri.v1, tri.v2)) {
          right_triangles.push_back(tri);
        }
      }

      assert(left_triangles.size() + right_triangles.size() >= triangles.size()
          && "geometry has disappeared");

      iter.split(d);

      buildTree(iter.left(), left_bounding, left_triangles);
      buildTree(iter.right(), right_bounding, right_triangles);
    }
  }
}

#endif /* end of include guard: BUILD_HPP_BTVASI2S */
