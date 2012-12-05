#ifndef BOUNDINGMIDDLE_HPP_VL4QZK2H
#define BOUNDINGMIDDLE_HPP_VL4QZK2H

#include <array>
#include <vector>

namespace kdtree {
  namespace build {
    void boundingMiddleBuildTree(KdNodeLinked* node, const std::vector<const Triangle*>& triangles, Axis axis, size_t depth) {
      constexpr std::array<Axis, 3> next = {{ Y, Z, X }};

      assert(node != nullptr);
      assert(!triangles.empty());

      if (depth >= 4 || triangles.size() <= 3) {
        node->type = Leaf;

        node->leaf.triangles = new std::vector<const Triangle*>();
        for (const Triangle* tri : triangles) {
          assert(tri != nullptr);
          node->leaf.triangles->push_back(tri);
        }
      } else {
        math::Aabb bounding = helpers::findBounding(triangles);
        float d = helpers::middle(
            helpers::swizzle(bounding.min, axis),
            helpers::swizzle(bounding.max, axis));

        node->type = Split;

        node->split.axis     = axis;
        node->split.distance = d;

        node->split.left  = new KdNodeLinked;
        node->split.right = new KdNodeLinked;

        std::vector<const Triangle*> left, right;
        for (const Triangle* tri : triangles) {
          if (helpers::containsLeft(tri, d, axis)) {
            assert(tri != nullptr);
            left.push_back(tri);
          }

          if (helpers::containsRight(tri, d, axis)) {
            assert(tri != nullptr);
            right.push_back(tri);
          }
        }

        assert(left.size() + right.size() >= triangles.size() && "geometry has disappeared");

        boundingMiddleBuildTree(node->split.left, left, next[axis], depth + 1);
        boundingMiddleBuildTree(node->split.right, right, next[axis], depth + 1);
      }
    }
  }
}

#endif /* end of include guard: BOUNDINGMIDDLE_HPP_VL4QZK2H */
