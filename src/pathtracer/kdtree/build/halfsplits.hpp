#ifndef HALFSPLITS_HPP_7SDFN4CN
#define HALFSPLITS_HPP_7SDFN4CN

#include <array>
#include <vector>

namespace kdtree {
  namespace build {
    namespace detail {
      template <typename Iter>
      void halfSplitsBuildTreeHelper(Iter iter, math::Aabb bounding, const std::vector<Triangle>& triangles) {
        if (iter.depth() >= 5 || triangles.size() <= 3) {
          iter.leaf(triangles);
        } else {
          float d = helpers::middle(
              helpers::swizzle(bounding.min, iter.axis()),
              helpers::swizzle(bounding.max, iter.axis()));

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

          math::Aabb left_bounding(bounding);
          math::Aabb right_bounding(bounding);

          helpers::swizzle(left_bounding.max, iter.axis())  = d;
          helpers::swizzle(right_bounding.min, iter.axis()) = d;

          iter.split(d);

          halfSplitsBuildTreeHelper(iter.left(), left_bounding, left_triangles);
          halfSplitsBuildTreeHelper(iter.right(), right_bounding, right_triangles);
        }
      }
    }

    template <typename Iter>
    void halfSplitsBuildTree(Iter iter, const std::vector<Triangle>& triangles) {
      detail::halfSplitsBuildTreeHelper(iter, helpers::findBounding(triangles), triangles);
    }
  }
}

#endif /* end of include guard: HALFSPLITS_HPP_7SDFN4CN */
