#ifndef KDTREE_HPP_3F5JNSBC
#define KDTREE_HPP_3F5JNSBC

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

      enum NodeType { Split, Leaf };

      struct SplitNode {
        Axis m_axis;
        float m_distance;

        KdNodeLinked* m_left;
        KdNodeLinked* m_right;
      };

      struct LeafNode {
        std::vector<Triangle>* m_triangles;
      };

      NodeType m_type;

      union {
        LeafNode m_leaf;
        SplitNode m_split;
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
            m_node->m_type = KdNodeLinked::Split;
            m_node->m_split.m_axis = m_axis;
            m_node->m_split.m_distance = d;
          }

          void leaf(const std::vector<Triangle>& triangles) {
            m_node->m_type = KdNodeLinked::Leaf;
            m_node->m_leaf.m_triangles = new std::vector<Triangle>();

            for (const Triangle& tri : triangles) {
              m_node->m_leaf.m_triangles->push_back(tri);
            }
          }

          BuildIter left() {
            constexpr std::array<Axis, 3> NEXT = {{ Y, Z, X }};

            m_node->m_split.m_left = new KdNodeLinked;
            return BuildIter(m_node->m_split.m_left, m_depth + 1, NEXT[m_axis]);
          }

          BuildIter right() {
            constexpr std::array<Axis, 3> NEXT = {{ Y, Z, X }};

            m_node->m_split.m_right = new KdNodeLinked;
            return BuildIter(m_node->m_split.m_right, m_depth + 1, NEXT[m_axis]);
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
            return node->m_type == KdNodeLinked::Leaf;
          }

          bool isSplit() const {
            return node->m_type == KdNodeLinked::Split;
          }

          Axis axis() const {
            assert(node->m_type == KdNodeLinked::Split);
            return node->m_split.m_axis;
          }

          float split() const {
            assert(node->m_type == KdNodeLinked::Split);
            return node->m_split.m_distance;
          }

          Iterator left() const {
            assert(node->m_type == KdNodeLinked::Split);
            return Iterator(node->m_split.m_left);
          }

          Iterator right() const {
            assert(node->m_type == KdNodeLinked::Split);
            return Iterator(node->m_split.m_right);
          }

          const std::vector<Triangle>& triangles() const {
            assert(node->m_type == KdNodeLinked::Leaf);
            return *node->m_leaf.m_triangles;
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
