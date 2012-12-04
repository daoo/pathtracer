#ifndef KDTREE_HPP_3F5JNSBC
#define KDTREE_HPP_3F5JNSBC

#include "kdtree/node.hpp"
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

      struct SplitNode {
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
        LeafNode leaf;
        SplitNode split;
      };

      class IteratorLinked {
        public:
          IteratorLinked(KdNodeLinked* n) : node(n) {
            assert(n != nullptr);
          }

          bool isLeaf() const {
            return node->type == Leaf;
          }

          bool isSplit() const {
            return node->type == Split;
          }

          Axis axis() const {
            assert(node->type == Split);
            return node->split.axis;
          }

          float split() const {
            assert(node->type == Split);
            return node->split.distance;
          }

          IteratorLinked left() const {
            assert(node->type == Split);
            return IteratorLinked(node->split.left);
          }

          IteratorLinked right() const {
            assert(node->type == Split);
            return IteratorLinked(node->split.right);
          }

          const std::vector<const Triangle*>& triangles() const {
            assert(node->type == Leaf);
            return *node->leaf.triangles;
          }

        private:
          const KdNodeLinked* node;
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
