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
    if (iter.depth() >= 12 || triangles.size() <= 10) {
      iter.leaf(triangles);
    } else {
      glm::vec3 new_size = bounding.half;
      helpers::swizzle(new_size, iter.axis()) = helpers::swizzle(bounding.half / 2.0f, iter.axis());

      math::Aabb left_bounding({ bounding.center, new_size });
      math::Aabb right_bounding({ bounding.center, new_size });

      helpers::swizzle(left_bounding.center, iter.axis())  -= helpers::swizzle(new_size, iter.axis());
      helpers::swizzle(right_bounding.center, iter.axis()) += helpers::swizzle(new_size, iter.axis());

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

      float d = helpers::swizzle(bounding.center, iter.axis());
      iter.split(d);

      buildTree(iter.left(), left_bounding, left_triangles);
      buildTree(iter.right(), right_bounding, right_triangles);
    }
  }
}

#endif /* end of include guard: BUILD_HPP_BTVASI2S */
