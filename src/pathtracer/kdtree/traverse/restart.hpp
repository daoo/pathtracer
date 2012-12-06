#ifndef RESTARTTRAVERSER_HPP_WKMTPG5O
#define RESTARTTRAVERSER_HPP_WKMTPG5O

#include "intersection.hpp"
#include "kdtree/util.hpp"
#include "math/ray.hpp"
#include "triangle.hpp"

namespace kdtree {
  namespace traverse {
    template <typename Tree>
    bool restartSearchTree(const Tree& tree, math::Ray& ray, Intersection& isect) {
      typename Tree::TraverseIter iter(tree);

      const float initial_maxt = ray.maxt;

      float mint = ray.mint;
      float maxt = ray.maxt;

      while (true) {
        if (iter.isLeaf()) {
          bool hit = false;

          for (const Triangle& tri : iter.triangles()) {
            hit |= intersects(tri, ray, isect);
          }

          if (hit && ray.maxt < maxt) {
            return true;
          } else if (maxt == initial_maxt) {
            return false;
          } else {
            iter = typename Tree::TraverseIter(tree);
            mint = maxt;
            maxt = initial_maxt;
          }
        } else if (iter.isSplit()) {
          float p = iter.split();

          float o = helpers::swizzle(ray.origin, iter.axis());
          float d = helpers::swizzle(ray.direction, iter.axis());

          float t = (p - o) / d;

          typename Tree::TraverseIter first(iter.left());
          typename Tree::TraverseIter second(iter.right());
          helpers::order(d, first, second);

          if (t >= maxt) {
            iter = first;
          } else if (t <= mint) {
            iter = second;
          } else {
            iter = first;
            maxt = t;
          }
        }
      }

      assert(false && "If this happens, something went very wrong.");
      return false;
    }
  }
}

#endif /* end of include guard: RESTARTTRAVERSER_HPP_WKMTPG5O */
