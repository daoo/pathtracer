#ifndef BUILD_HPP_BTVASI2S
#define BUILD_HPP_BTVASI2S

#include "math/aabb.hpp"
#include "math/tribox.hpp"
#include "pathtracer/kdtree/util.hpp"
#include "pathtracer/triangle.hpp"

#include <array>
#include <vector>

namespace kdtree {
  template <typename Iter>
  void buildTree(Iter iter, const math::Aabb& bounding,
      const std::vector<Triangle>& triangles) {
    constexpr float epsilon = 0.0000001f;
    const glm::vec3 vec_epsilon(epsilon);

    if (iter.depth() >= 20 || triangles.size() <= 10) {
      iter.leaf(triangles);
    } else {
      glm::vec3 offset(0, 0, 0);

      float d = helpers::swizzle(bounding.center, iter.axis());

      math::Aabb left_bounding;
      math::Aabb right_bounding;

      helpers::aabbFromSplit(bounding, iter.axis(), d,
          left_bounding, right_bounding);

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
