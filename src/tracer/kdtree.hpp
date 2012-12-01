#ifndef KDTREE_HPP_3F5JNSBC
#define KDTREE_HPP_3F5JNSBC

#include "math/ray.hpp"
#include "tracer/scene.hpp"

#include <glm/glm.hpp>
#include <vector>

enum Axis {
  X = 0, Y = 1, Z = 2
};

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
};

struct KdTree {
  KdNode* root;
};

KdTree buildTree(const Scene&);

bool intersectsAll(const KdTree&, math::Ray&, Intersection&);

#endif /* end of include guard: KDTREE_HPP_3F5JNSBC */
