#ifndef STACK_HPP_5DYINPFY
#define STACK_HPP_5DYINPFY

#include "intersection.hpp"
#include "kdtree/node.hpp"
#include "kdtree/util.hpp"
#include "math/ray.hpp"
#include "triangle.hpp"

#include <stack>
#include <tuple>

namespace kdtree {
  namespace traverse {
    namespace detail {
      template <typename Iter>
      bool searchNode(Iter iter, std::stack<std::tuple<Iter, float, float>> stack,
          float mint, float maxt, math::Ray& ray, Intersection& isect) {
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
            std::tuple<Iter, float, float> tmp = stack.top();
            stack.pop();
            return searchNode(std::get<0>(tmp), stack, std::get<1>(tmp), std::get<2>(tmp), ray, isect);
          }
        } else if (iter.isSplit()) {
          float p = iter.split();

          float o = helpers::swizzle(ray.origin, iter.axis());
          float d = helpers::swizzle(ray.direction, iter.axis());

          float t = (p - o) / d;

          Iter first;
          Iter second;
          helpers::order(d, iter.left(), iter.right(), first, second);

          if (t >= maxt) {
            return searchNode(first, stack, mint, maxt, ray, isect);
          } else if (t <= mint) {
            return searchNode(second, stack, mint, maxt, ray, isect);
          } else {
            stack.push(std::make_tuple(second, t, maxt));
            return searchNode(first, stack, mint, t, ray, isect);
          }
        }

        assert(false && "Incomplete if");
        return false;
      }
    }

    template <typename Tree>
    bool stackSearchTree(const Tree& tree, math::Ray& ray, Intersection& isect) {
      return detail::searchNode(typename Tree::Iterator(tree),
          std::stack<std::tuple<typename Tree::Iterator, float, float>>(),
          ray.mint, ray.maxt, ray, isect);
    }
  }
}

#endif /* end of include guard: STACK_HPP_5DYINPFY */
