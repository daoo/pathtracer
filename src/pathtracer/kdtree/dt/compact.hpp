#ifndef LINKED_HPP_DGVBYSLC
#define LINKED_HPP_DGVBYSLC

#include "kdtree/util.hpp"
#include "math/ray.hpp"
#include "triangle.hpp"

#include <array>
#include <glm/glm.hpp>
#include <vector>

namespace kdtree {
  class KdTreeLinked {
    public:
      KdTreeLinked() : m_root(new Node) { }
      ~KdTreeLinked() { delete m_root; }

    private:
      class Node {
        public:
          Node() { }
          ~Node() {
            if (m_type == Leaf) {
              delete m_leaf.m_triangles;
            } else if (m_type == Split) {
              delete m_split.m_left;
              delete m_split.m_right;
            }
          }

          constexpr size_t NODE_TYPE_MASK = 0x1; // 0b1

          constexpr size_t AXIS_MASK      = 0xE; // 0b1110
          constexpr size_t DISTANCE_MASK  = ~(NODE_TYPE_MASK + AXIS_MASK);
          constexpr size_t CHILD_PTR_MASK = ~(NODE_TYPE_MASK + AXIS_MASK);

          constexpr bool is_leaf(int64_t bitfield) {
            return bitfield & NODE_TYPE_MASK == 0;
          }

          constexpr bool is_split(int64_t bitfield) {
            return bitfield & NODE_TYPE_MASK == 1;
          }

          constexpr float distance(int64_t bitfield) {
            return bitfield &
          }

          /**
           *
           * From MSB to LSB:
           *   - 60 bits of data
           *   - 3 bits Axis (for splits)
           *   - 1 bit Node type
           */
          size_t bitfield;
          size_t pointer;
      };

      Node* m_root;

      KdTreeLinked(const KdTreeLinked&);
      KdTreeLinked& operator=(const KdTreeLinked&);

    public:
      class BuildIter {
        public:
          BuildIter(KdTreeLinked& tree) :
            m_node(tree.m_root), m_depth(0), m_axis(X) { }

          Axis axis() {
            return m_axis;
          }

          size_t depth() {
            return m_depth;
          }

          void split(float d) {
            m_node->m_type = Node::Split;
            m_node->m_split.m_axis = m_axis;
            m_node->m_split.m_distance = d;
          }

          void leaf(const std::vector<Triangle>& triangles) {
            m_node->m_type = Node::Leaf;

            if (triangles.empty()) {
              m_node->m_leaf.m_triangles = nullptr;
            } else {
              m_node->m_leaf.m_triangles = new std::vector<Triangle>();
              for (const Triangle& tri : triangles) {
                m_node->m_leaf.m_triangles->push_back(tri);
              }
            }
          }

          BuildIter left() {
            constexpr std::array<Axis, 3> NEXT = {{ Y, Z, X }};

            m_node->m_split.m_left = new Node;
            return BuildIter(m_node->m_split.m_left, m_depth + 1, NEXT[m_axis]);
          }

          BuildIter right() {
            constexpr std::array<Axis, 3> NEXT = {{ Y, Z, X }};

            m_node->m_split.m_right = new Node;
            return BuildIter(m_node->m_split.m_right, m_depth + 1, NEXT[m_axis]);
          }

        private:
          Node* m_node;
          size_t m_depth;
          Axis m_axis;

          BuildIter(Node* node, size_t depth, Axis axis) :
              m_node(node), m_depth(depth), m_axis(axis) {
            assert(node != nullptr);
          }
      };

      class TraverseIter {
        public:
          TraverseIter(const KdTreeLinked& tree) : m_node(tree.m_root) { }

          bool isLeaf() const {
            return m_node->m_type == Node::Leaf;
          }

          bool hasTriangles() const {
            assert(m_node->m_type == Node::Leaf);
            return m_node->m_leaf.m_triangles != nullptr;
          }

          bool isSplit() const {
            return m_node->m_type == Node::Split;
          }

          Axis axis() const {
            assert(m_node->m_type == Node::Split);
            return m_node->m_split.m_axis;
          }

          float split() const {
            assert(m_node->m_type == Node::Split);
            return m_node->m_split.m_distance;
          }

          TraverseIter left() const {
            assert(m_node->m_type == Node::Split);
            return TraverseIter(m_node->m_split.m_left);
          }

          TraverseIter right() const {
            assert(m_node->m_type == Node::Split);
            return TraverseIter(m_node->m_split.m_right);
          }

          const std::vector<Triangle>& triangles() const {
            assert(m_node->m_type == Node::Leaf);
            assert(m_node->m_leaf.m_triangles != nullptr);
            return *m_node->m_leaf.m_triangles;
          }

        private:
          const Node* m_node;

          TraverseIter(Node* n) : m_node(n) {
            assert(n != nullptr);
          }
      };
  };
}

#endif /* end of include guard: LINKED_HPP_DGVBYSLC */