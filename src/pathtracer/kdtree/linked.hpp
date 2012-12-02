#ifndef KDTREE_HPP_3F5JNSBC
#define KDTREE_HPP_3F5JNSBC

#include "kdtree/util.hpp"
#include "math/ray.hpp"
#include "triangle.hpp"

#include <glm/glm.hpp>
#include <ostream>
#include <vector>

namespace kdtree {
  class KdNodeLinked {
    public:
      KdNodeLinked();
      ~KdNodeLinked();

      enum NodeType { Parent, Leaf };

      struct ParentNode {
        Axis axis;
        float distance;

        KdNodeLinked* left;
        KdNodeLinked* right;
      };

      struct LeafNode {
        std::vector<const Triangle*>* triangles;
      };

      NodeType type;

      union {
        ParentNode parent;
        LeafNode leaf;
      };
  };

  class KdTreeLinked {
    public:
      KdNodeLinked* root;

      KdTreeLinked();
      ~KdTreeLinked();

    private:
      KdTreeLinked(const KdTreeLinked&);
      KdTreeLinked& operator=(const KdTreeLinked&);
  };

  void buildKdTreeLinked(KdTreeLinked&, const std::vector<Triangle>&);

  bool intersects(const KdTreeLinked&, math::Ray&, Intersection&);
  bool intersects(const KdTreeLinked&, const math::Ray&);

  void print(std::ostream&, const KdTreeLinked&);
}

#endif /* end of include guard: KDTREE_HPP_3F5JNSBC */
