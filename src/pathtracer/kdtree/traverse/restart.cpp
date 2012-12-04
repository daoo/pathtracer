#include "restart.hpp"

namespace kdtree {
  namespace {
    struct State {
      State(KdNodeLinked* root, math::Ray& r, Intersection& i) : root(root), start_maxt(r.maxt), ray(r), isect(i) { }

      const KdNodeLinked* root;
      const float start_maxt;
      math::Ray& ray;
      Intersection& isect;
    };

    bool searchNode(State&, const KdNodeLinked*, float, float);
    bool searchLeaf(State&, const KdNodeLinked::LeafNode&, float);
    bool continueSearch(State&, float);
    bool searchSplit(State&, const KdNodeLinked::SplitNode&, float, float);

    bool searchNode(State& state, const KdNodeLinked* node, float mint, float maxt) {
      assert(node != nullptr);

      if (node->type == Leaf) {
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
        return continueSearch(state, maxt);
      }
    }

    bool continueSearch(State& state, float maxt) {
      if (maxt == state.start_maxt) {
        return false;
      } else {
        return searchNode(state, state.root, maxt, state.start_maxt);
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

      if (t >= maxt) {
        return searchNode(state, first, mint, maxt);
      } else if (t <= mint) {
        return searchNode(state, second, mint, maxt);
      } else {
        return searchNode(state, first, mint, t);
      }
    }
  }

  namespace traverse {
    namespace restart {
      bool searchTree(const KdTreeLinked& tree, math::Ray& ray,
          Intersection& isect) {
        assert(tree.root != nullptr);

        State state(tree.root, ray, isect);
        return searchNode(state, tree.root, ray.mint, ray.maxt);
      }
    }
  }
}
