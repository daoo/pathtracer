#include "stacktraverser.hpp"

#include <stack>
#include <tuple>

namespace kdtree {
  namespace {
    struct State {
      State(math::Ray& r, Intersection& i) : stack(), ray(r), isect(i) { }

      std::stack<std::tuple<const KdNodeLinked*, float, float>> stack;
      math::Ray& ray;
      Intersection& isect;
    };

    bool searchNode(State&, const KdNodeLinked*, float, float);
    bool searchLeaf(State&, const KdNodeLinked::LeafNode&, float);
    bool continueSearch(State&);
    bool searchSplit(State&, const KdNodeLinked::SplitNode&, float, float);

    bool searchNode(State& state, const KdNodeLinked* node, float mint, float maxt) {
      assert(node != nullptr);

      if (node->type == KdNodeLinked::Leaf) {
        return searchLeaf(state, node->leaf, maxt);
      } else /*if (node->type == KdNodeLinked::Split)*/ {
        return searchSplit(state, node->split, mint, maxt);
      }
    }

    bool searchLeaf(State& state, const KdNodeLinked::LeafNode& node, float maxt) {
      bool hit = false;

      std::vector<const Triangle*> tris = *node.triangles;
      for (size_t i = 0; i < tris.size(); ++i) {
        hit |= intersects(*(tris[i]), state.ray, state.isect);
      }

      if (hit && state.ray.maxt < maxt) {
        return true;
      } else {
        return continueSearch(state);
      }
    }

    bool continueSearch(State& state) {
      if (state.stack.empty()) {
        return false;
      } else {
        std::tuple<const KdNodeLinked*, float, float> tmp = state.stack.top();
        state.stack.pop();
        return searchNode(state, std::get<0>(tmp), std::get<1>(tmp), std::get<2>(tmp));
      }
    }

    bool searchSplit(State& state, const KdNodeLinked::SplitNode& node, float mint, float maxt) {
      float p = node.distance;

      float o = helpers::swizzle(state.ray.origin, node.axis);
      float d = helpers::swizzle(state.ray.direction, node.axis);

      float t = (p - o) / d;

      const KdNodeLinked* first;
      const KdNodeLinked* second;
      helpers::order(d, node.left, node.right, first, second);

      if (t >= maxt || t < 0) {
        return searchNode(state, first, mint, maxt);
      } else if (t <= mint) {
        return searchNode(state, second, mint, maxt);
      } else {
        state.stack.push(std::make_tuple(second, t, maxt));
        return searchNode(state, first, mint, t);
      }
    }
  }

  namespace traverse {
    namespace stack {
      bool searchTree(const KdTreeLinked& tree, math::Ray& ray,
          Intersection& isect) {
        assert(tree.root != nullptr);

        State state(ray, isect);
        return searchNode(state, tree.root, ray.mint, ray.maxt);
      }
    }
  }
}
