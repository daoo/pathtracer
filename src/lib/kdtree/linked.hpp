#ifndef KDTREE_HPP_3F5JNSBC
#define KDTREE_HPP_3F5JNSBC

#include "kdtree/util.hpp"
#include "math/ray.hpp"
#include "tracer/triangle.hpp"

#include <glm/glm.hpp>
#include <vector>

namespace kdtree {
  class KdNodeLinked {
    public:
      KdNodeLinked() { }

      Axis dir;
      float d;

      enum NodeType { Parent, Leaf };

      struct ParentNode {
        KdNodeLinked* left;
        KdNodeLinked* right;
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

  struct KdTreeLinked {
    KdNodeLinked* root;
  };

  KdTreeLinked buildKdTreeLinked(const std::vector<Triangle>&);

  bool intersects(const KdTreeLinked&, math::Ray&, Intersection&);
  bool intersects(const KdTreeLinked&, const math::Ray&);
}

#endif /* end of include guard: KDTREE_HPP_3F5JNSBC */
