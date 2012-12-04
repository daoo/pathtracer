#ifndef STACK_HPP_5DYINPFY
#define STACK_HPP_5DYINPFY

#include "intersection.hpp"
#include "kdtree/util.hpp"
#include "math/ray.hpp"
#include "triangle.hpp"

#include <stack>
#include <tuple>

namespace kdtree {
  namespace traverse {
    template <typename Tree>
    bool stackSearchTree(const Tree& tree, math::Ray& ray, Intersection& isect) {
      std::stack<std::tuple<typename Tree::Iterator, float, float>> stack;
      typename Tree::Iterator iter(tree);

      float mint = ray.mint;
      float maxt = ray.maxt;

      while (true) {
        if (iter.isLeaf()) {
          bool hit = false;

          for (const Triangle* tri : iter.triangles()) {
            hit |= intersects(*tri, ray, isect);
          }

          if (hit && ray.maxt < maxt) {
            return true;
          } else if (stack.empty()) {
            return false;
          } else {
            std::tuple<typename Tree::Iterator, float, float> tmp = stack.top();
            stack.pop();

            iter = std::get<0>(tmp);
            mint = std::get<1>(tmp);
            maxt = std::get<2>(tmp);
          }
        } else if (iter.isSplit()) {
          float p = iter.split();

          float o = helpers::swizzle(ray.origin, iter.axis());
          float d = helpers::swizzle(ray.direction, iter.axis());

          float t = (p - o) / d;

          typename Tree::Iterator first;
          typename Tree::Iterator second;
          helpers::order(d, iter.left(), iter.right(), first, second);

          if (t >= maxt) {
            iter = first;
          } else if (t <= mint) {
            iter = second;
          } else {
            stack.push(std::make_tuple(second, t, maxt));
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

#endif /* end of include guard: STACK_HPP_5DYINPFY */
