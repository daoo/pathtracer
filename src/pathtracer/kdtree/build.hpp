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
  void buildTree(Iter iter, math::Aabb bounding, const std::vector<Triangle>& triangles) {
    if (iter.depth() >= 10 || triangles.size() <= 5) {
      iter.leaf(triangles);
    } else {
      float d = helpers::swizzle(bounding.center, iter.axis());

      std::vector<Triangle> left_triangles, right_triangles;
      for (const Triangle& tri : triangles) {
        if (helpers::containsLeft(tri, d, iter.axis())) {
          left_triangles.push_back(tri);
        }

        if (helpers::containsRight(tri, d, iter.axis())) {
          right_triangles.push_back(tri);
        }
      }

      assert(left_triangles.size() + right_triangles.size() >= triangles.size()
          && "geometry has disappeared");

      glm::vec3 new_size = bounding.half / 2.0f;

      math::Aabb left_bounding({ bounding.center, new_size });
      math::Aabb right_bounding({ bounding.center, new_size });

      helpers::swizzle(left_bounding.center, iter.axis())  -= helpers::swizzle(new_size, iter.axis());
      helpers::swizzle(right_bounding.center, iter.axis()) += helpers::swizzle(new_size, iter.axis());

      iter.split(d);

      buildTree(iter.left(), left_bounding, left_triangles);
      buildTree(iter.right(), right_bounding, right_triangles);
    }
  }
}

#endif /* end of include guard: BUILD_HPP_BTVASI2S */
