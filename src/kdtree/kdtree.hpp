#ifndef KDTREE_HPP_3F5JNSBC
#define KDTREE_HPP_3F5JNSBC

#include "kdtree/util.hpp"
#include "math/ray.hpp"
#include "tracer/scene.hpp"

#include <glm/glm.hpp>
#include <vector>

namespace kdtree {
  enum NodeType {
    Parent, Leaf
  };

  class KdNode {
    public:
      KdNode() { }

      Axis dir;
      float d;

      struct ParentNode {
        KdNode* left;
        KdNode* right;
      };

      struct LeafNode {
        std::vector<const Triangle*> triangles;
      };

      NodeType type;

      union {
        ParentNode parent;
        LeafNode leaf;
      };

      static_assert(sizeof(LeafNode) > sizeof(ParentNode), "test");
  };

  struct LinkedKdTree {
    KdNode* root;
  };

  KdTree buildTree(const Scene&);

  bool intersectsAll(const KdTree&, math::Ray&, Intersection&);
}

#endif /* end of include guard: KDTREE_HPP_3F5JNSBC */
