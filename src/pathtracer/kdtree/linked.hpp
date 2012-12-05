#ifndef KDTREE_HPP_3F5JNSBC
#define KDTREE_HPP_3F5JNSBC

#include "kdtree/node.hpp"
#include "kdtree/util.hpp"
#include "math/ray.hpp"
#include "triangle.hpp"

#include <array>
#include <glm/glm.hpp>
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
        std::vector<Triangle>* triangles;
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

    private:
      KdTreeLinked(const KdTreeLinked&);
      KdTreeLinked& operator=(const KdTreeLinked&);

    public:
      class BuildIter {
        public:
          BuildIter(KdNodeLinked* n, size_t depth, Axis axis) : m_axis(axis), m_depth(depth), m_node(n) {
            assert(n != nullptr);
          }

          BuildIter(KdTreeLinked& tree) : m_axis(X), m_depth(0), m_node(tree.root) {
            assert(tree.root != nullptr);
          }

          Axis axis() {
            return m_axis;
          }

          size_t depth() {
            return m_depth;
          }

          void split(float d) {
            m_node->type = Split;
            m_node->split.axis = m_axis;
            m_node->split.distance = d;
          }

          void leaf(const std::vector<Triangle>& triangles) {
            m_node->type = Leaf;
            m_node->leaf.triangles = new std::vector<Triangle>();

            for (const Triangle& tri : triangles) {
              m_node->leaf.triangles->push_back(tri);
            }
          }

          BuildIter left() {
            constexpr std::array<Axis, 3> NEXT = {{ Y, Z, X }};

            m_node->split.left = new KdNodeLinked;
            return BuildIter(m_node->split.left, m_depth + 1, NEXT[m_axis]);
          }

          BuildIter right() {
            constexpr std::array<Axis, 3> NEXT = {{ Y, Z, X }};

            m_node->split.right = new KdNodeLinked;
            return BuildIter(m_node->split.right, m_depth + 1, NEXT[m_axis]);
          }

        private:
          Axis m_axis;
          size_t m_depth;
          KdNodeLinked* m_node;
      };

      class Iterator {
        public:
          Iterator(const KdTreeLinked& tree) : node(tree.root) {
            assert(tree.root != nullptr);
          }

          Iterator(const Iterator& iter) : node(iter.node) { }

          Iterator& operator=(const Iterator& iter) noexcept {
            node = iter.node;
            return *this;
          }

          Iterator& operator=(Iterator&& iter) noexcept {
            node = std::move(iter.node);
            iter.node = nullptr;
            return *this;
          }

          Iterator(Iterator&& iter) noexcept {
            node = std::move(iter.node);
            iter.node = nullptr;
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

          const std::vector<Triangle>& triangles() const {
            assert(node->type == Leaf);
            return *node->leaf.triangles;
          }

        private:
          const KdNodeLinked* node;

          Iterator(KdNodeLinked* n) : node(n) {
            assert(n != nullptr);
          }
      };
  };

  void buildKdTreeLinked(KdTreeLinked&, const std::vector<Triangle>&);
}

#endif /* end of include guard: KDTREE_HPP_3F5JNSBC */
