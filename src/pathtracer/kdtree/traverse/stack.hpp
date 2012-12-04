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
      struct State {
        State(math::Ray& r, Intersection& i) : stack(), ray(r), isect(i) { }

        std::stack<std::tuple<Iter, float, float>> stack;
        math::Ray& ray;
        Intersection& isect;
      };

      template <typename Iter>
      bool searchNode(State<Iter>& state, Iter iter, float mint, float maxt) {
        if (iter.isLeaf()) {
          bool hit = false;

          for (const Triangle* tri : iter.triangles()) {
            hit |= intersects(*tri, state.ray, state.isect);
          }

          if (hit && state.ray.maxt < maxt) {
            return true;
          } else if (state.stack.empty()) {
            return false;
          } else {
            std::tuple<Iter, float, float> tmp = state.stack.top();
            state.stack.pop();
            return searchNode(state, std::get<0>(tmp), std::get<1>(tmp), std::get<2>(tmp));
          }
        } else if (iter.isSplit()) {
          float p = iter.split();

          float o = helpers::swizzle(state.ray.origin, iter.axis());
          float d = helpers::swizzle(state.ray.direction, iter.axis());

          float t = (p - o) / d;

          Iter first;
          Iter second;
          helpers::order(d, iter.left(), iter.right(), first, second);

          if (t >= maxt) {
            return searchNode(state, first, mint, maxt);
          } else if (t <= mint) {
            return searchNode(state, second, mint, maxt);
          } else {
            state.stack.push(std::make_tuple(second, t, maxt));
            return searchNode(state, first, mint, t);
          }
        }

        assert(false && "Incomplete if");
        return false;
      }
    }

    template <typename Tree>
    bool stackSearchTree(const Tree& tree, math::Ray& ray, Intersection& isect) {
      detail::State<typename Tree::Iterator> state(ray, isect);
      return detail::searchNode(state, typename Tree::Iterator(tree), ray.mint, ray.maxt);
    }
  }
}

#endif /* end of include guard: STACK_HPP_5DYINPFY */
