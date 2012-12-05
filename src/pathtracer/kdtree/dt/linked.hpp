#ifndef KDTREE_HPP_3F5JNSBC
#define KDTREE_HPP_3F5JNSBC

#include "kdtree/util.hpp"
#include "math/ray.hpp"
#include "triangle.hpp"

#include <array>
#include <glm/glm.hpp>
#include <vector>

namespace kdtree {
  class KdTreeLinked {
    public:
      KdTreeLinked() : m_root(new KdNodeLinked) { }
      ~KdTreeLinked() { delete m_root; }

    private:
      class KdNodeLinked {
        public:
          KdNodeLinked() { }
          ~KdNodeLinked() {
            if (m_type == Leaf) {
              delete m_leaf.m_triangles;
            } else if (m_type == Split) {
              delete m_split.m_left;
              delete m_split.m_right;
            }
          }

          enum NodeType { Split, Leaf };

          struct SplitNode {
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

      static constexpr Axis next(Axis axis) {
        return axis == X ? Y : axis == Y ? Z : X;
      }

      Axis m_starting_axis;
      KdNodeLinked* m_root;

      KdTreeLinked(const KdTreeLinked&);
      KdTreeLinked& operator=(const KdTreeLinked&);

    public:
      class BuildIter {
        public:
          BuildIter(KdNodeLinked* n, size_t depth, Axis axis) : m_axis(axis), m_depth(depth), m_node(n) {
            assert(n != nullptr);
          }

          BuildIter(KdTreeLinked& tree) : m_axis(X), m_depth(0), m_node(tree.m_root) { }

          Axis axis() {
            return m_axis;
          }

          size_t depth() {
            return m_depth;
          }

          void split(float d) {
            m_node->m_type = KdNodeLinked::Split;
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
            m_node->m_split.m_left = new KdNodeLinked;
            return BuildIter(m_node->m_split.m_left, m_depth + 1, next(m_axis));
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

      class TraverseIter {
        public:
          TraverseIter(const KdTreeLinked& tree) : m_node(tree.m_root), m_axis(tree.m_starting_axis) { }

          TraverseIter(const TraverseIter& iter) : m_node(iter.m_node), m_axis(iter.m_axis) { }

          TraverseIter& operator=(const TraverseIter& iter) noexcept {
            m_node = iter.m_node;
            m_axis = iter.m_axis;
            return *this;
          }

          TraverseIter& operator=(TraverseIter&& iter) noexcept {
            m_node = std::move(iter.m_node);
            m_axis = std::move(iter.m_axis);
            iter.m_node = nullptr;
            return *this;
          }

          TraverseIter(TraverseIter&& iter) noexcept {
            m_node = std::move(iter.m_node);
            m_axis = std::move(iter.m_axis);
            iter.m_node = nullptr;
          }

          bool isLeaf() const {
            return m_node->m_type == KdNodeLinked::Leaf;
          }

          bool isSplit() const {
            return m_node->m_type == KdNodeLinked::Split;
          }

          Axis axis() const {
            assert(m_node->m_type == KdNodeLinked::Split);
            return m_axis;
          }

          float split() const {
            assert(m_node->m_type == KdNodeLinked::Split);
            return m_node->m_split.m_distance;
          }

          TraverseIter left() const {
            assert(m_node->m_type == KdNodeLinked::Split);
            return TraverseIter(m_node->m_split.m_left, next(m_axis));
          }

          TraverseIter right() const {
            assert(m_node->m_type == KdNodeLinked::Split);
            return TraverseIter(m_node->m_split.m_right, next(m_axis));
          }

          const std::vector<Triangle>& triangles() const {
            assert(m_node->m_type == KdNodeLinked::Leaf);
            return *m_node->m_leaf.m_triangles;
          }

        private:
          const KdNodeLinked* m_node;
          Axis m_axis;

          TraverseIter(KdNodeLinked* n, Axis axis) : m_node(n), m_axis(axis) {
            assert(n != nullptr);
          }
      };
  };
}

#endif /* end of include guard: KDTREE_HPP_3F5JNSBC */
