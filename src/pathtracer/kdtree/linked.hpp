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
  };

  class KdTreeLinked {
    public:
      KdNodeLinked* root;

      KdTreeLinked();
      ~KdTreeLinked();

      class Iterator {
        public:
          Iterator() { }

          Iterator& operator=(const Iterator& iter) {
            if (this != &iter) {
              node = iter.node;
            }

            return *this;
          }

          Iterator(const KdTreeLinked& tree) : node(tree.root) {
            assert(tree.root != nullptr);
          }

          Iterator(KdNodeLinked* n) : node(n) {
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

          Iterator left() const {
            assert(node->type == Split);
            return Iterator(node->split.left);
          }

          Iterator right() const {
            assert(node->type == Split);
            return Iterator(node->split.right);
          }

          const std::vector<const Triangle*>& triangles() const {
            assert(node->type == Leaf);
            return *node->leaf.triangles;
          }

        private:
          const KdNodeLinked* node;
      };

    private:
      KdTreeLinked(const KdTreeLinked&);
      KdTreeLinked& operator=(const KdTreeLinked&);
  };

  void buildKdTreeLinked(KdTreeLinked&, const std::vector<Triangle>&);

  void print(std::ostream&, const KdTreeLinked&);
}

#endif /* end of include guard: KDTREE_HPP_3F5JNSBC */
