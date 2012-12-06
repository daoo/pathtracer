#ifndef ARRAY_HPP_BBXOECNY
#define ARRAY_HPP_BBXOECNY

#include "kdtree/util.hpp"
#include "math/ray.hpp"
#include "triangle.hpp"

#include <array>
#include <glm/glm.hpp>
#include <vector>

namespace kdtree {
  class KdTreeArray {
    public:
      KdTreeArray() : m_nodes() { }
      ~KdTreeArray() { }

    private:
      class Node {
        public:
          Node() { }
          ~Node() {
            if (m_type == Leaf) {
              delete m_leaf.m_triangles;
            }
          }

          enum NodeType { Split, Leaf };

          struct SplitNode {
            Axis m_axis;
            float m_distance;
          };

          struct LeafNode {
            /**
             * List of triangles in this leaf.
             *
             * Null pointer means this leaf is empty.
             */
            std::vector<Triangle>* m_triangles;
          };

          NodeType m_type;

          union {
            LeafNode m_leaf;
            SplitNode m_split;
          };
      };

      std::vector<Node> m_nodes;

      KdTreeArray(const KdTreeArray&);
      KdTreeArray& operator=(const KdTreeArray&);

    public:
      class BuildIter {
        public:
          BuildIter(KdTreeArray& tree) :
            m_nodes(tree.m_nodes), m_index(0), m_depth(0), m_axis(X) { }

          Axis axis() {
            return m_axis;
          }

          size_t depth() {
            return m_depth;
          }

          void split(float d) {
            m_nodes.reserve(m_index);

            m_nodes[m_index].m_type = Node::Split;
            m_nodes[m_index].m_split.m_axis = m_axis;
            m_nodes[m_index].m_split.m_distance = d;
          }

          void leaf(const std::vector<Triangle>& triangles) {
            m_nodes.reserve(m_index);

            m_nodes[m_index].m_type = Node::Leaf;

            if (triangles.empty()) {
              m_nodes[m_index].m_leaf.m_triangles = nullptr;
            } else {
              m_nodes[m_index].m_leaf.m_triangles = new std::vector<Triangle>();
              for (const Triangle& tri : triangles) {
                m_nodes[m_index].m_leaf.m_triangles->push_back(tri);
              }
            }
          }

          BuildIter left() {
            constexpr std::array<Axis, 3> NEXT = {{ Y, Z, X }};
            return BuildIter(m_nodes, (m_index << 1) + 1, m_depth + 1, NEXT[m_axis]);
          }

          BuildIter right() {
            constexpr std::array<Axis, 3> NEXT = {{ Y, Z, X }};
            return BuildIter(m_nodes, (m_index << 1) + 2, m_depth + 1, NEXT[m_axis]);
          }

        private:
          std::vector<Node>& m_nodes;

          size_t m_index;
          size_t m_depth;
          Axis m_axis;

          BuildIter(std::vector<Node>& nodes, size_t index, size_t depth, Axis axis) :
              m_nodes(nodes), m_index(index), m_depth(depth), m_axis(axis) {
            assert(index < nodes.size());
          }
      };

      class TraverseIter {
        public:
          TraverseIter(const KdTreeArray& tree) :
            m_nodes(tree.m_nodes), m_node(&tree.m_nodes[0]), m_index(0) { }

          TraverseIter& operator=(const TraverseIter& iter) {
            assert(this != &iter);

            // Note that we do not allow an iterator to change the tree it iterates
            assert(&m_nodes == &iter.m_nodes);

            m_node  = iter.m_node;
            m_index = iter.m_index;

            return *this;
          }

          TraverseIter(TraverseIter&& iter) :
              m_nodes(std::move(iter.m_nodes)), m_node(std::move(iter.m_node)),
              m_index(std::move(iter.m_index)) {
            iter.m_node = nullptr;
          }

          TraverseIter& operator=(TraverseIter&& iter) {
            assert(this != &iter);
            assert(&m_nodes == &iter.m_nodes);

            m_node  = std::move(iter.m_node);
            m_index = std::move(iter.m_index);

            iter.m_node = nullptr;

            return *this;
          }

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
            return TraverseIter(m_nodes, (m_index << 1) + 1);
          }

          TraverseIter right() const {
            assert(m_node->m_type == Node::Split);
            return TraverseIter(m_nodes, (m_index << 1) + 2);
          }

          const std::vector<Triangle>& triangles() const {
            assert(m_node->m_type == Node::Leaf);
            return *m_node->m_leaf.m_triangles;
          }

        private:
          const std::vector<Node>& m_nodes;
          const Node* m_node;
          size_t m_index;

          TraverseIter(const std::vector<Node>& nodes, size_t index) :
              m_nodes(nodes), m_node(&nodes[index]), m_index(index) { }
      };
  };
}

#endif /* end of include guard: ARRAY_HPP_BBXOECNY */
