#ifndef BOUNDINGMIDDLE_HPP_VL4QZK2H
#define BOUNDINGMIDDLE_HPP_VL4QZK2H

#include "kdtree/util.hpp"
#include "math/aabb.hpp"
#include "triangle.hpp"

#include <array>
#include <vector>

namespace kdtree {
  namespace build {
    template <typename Iter>
    void boundingMiddleBuildTree(Iter iter, const std::vector<Triangle>& triangles) {
      if (iter.depth() >= 3 || triangles.size() <= 3) {
        iter.leaf(triangles);
      } else {
        math::Aabb bounding = helpers::findBounding(triangles);
        float d = helpers::middle(
            helpers::swizzle(bounding.min, iter.axis()),
            helpers::swizzle(bounding.max, iter.axis()));

        std::vector<Triangle> left, right;
        for (const Triangle& tri : triangles) {
          if (helpers::containsLeft(tri, d, iter.axis())) {
            left.push_back(tri);
          }

          if (helpers::containsRight(tri, d, iter.axis())) {
            right.push_back(tri);
          }
        }

        assert(left.size() + right.size() >= triangles.size()
            && "geometry has disappeared");

        iter.split(d);

        boundingMiddleBuildTree(iter.left(), left);
        boundingMiddleBuildTree(iter.right(), right);
      }
    }
  }
}

#endif /* end of include guard: BOUNDINGMIDDLE_HPP_VL4QZK2H */
