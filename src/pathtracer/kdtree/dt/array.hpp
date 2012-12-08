#ifndef ARRAY_HPP_BBXOECNY
#define ARRAY_HPP_BBXOECNY

#include "pathtracer/kdtree/util.hpp"
#include "pathtracer/math/ray.hpp"
#include "pathtracer/triangle.hpp"

#include <array>
#include <vector>

namespace kdtree {
  class KdTreeArray {
    public:
      KdTreeArray() : m_nodes() { }
      ~KdTreeArray() {
        for (Node node : m_nodes) {
          if (node.m_type == Node::Leaf) {
            delete node.m_leaf.m_triangles;
          }
        }
      }

      class Node {
        public:
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

          Node() { }
          Node(LeafNode&& data) : m_type(Leaf), m_leaf(data) {
            data.m_triangles = nullptr;
          }

          Node(SplitNode&& data) : m_type(Split), m_split(data) { }

          ~Node() { }

          NodeType m_type;

          union {
            LeafNode m_leaf;
            SplitNode m_split;
          };
      };

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

          /**
           * Create a split node.
           */
          void split(float d) {
            setNode(Node({m_axis, d}));
          }

          /**
           * Create a leaf node.
           */
          void leaf(const std::vector<Triangle>& triangles) {
            std::vector<Triangle>* ts;
            if (triangles.empty()) {
              ts = nullptr;
            } else {
              ts = new std::vector<Triangle>();
              for (const Triangle& tri : triangles) {
                ts->push_back(tri);
              }
            }

            setNode(Node::LeafNode{ts});
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
              m_nodes(nodes), m_index(index), m_depth(depth), m_axis(axis) { }

          /**
           * Set the current node.
           */
          void setNode(Node&& node) {
            // When a build iter is created for some node, that node does not
            // acctually exists in the underlying vector.

            if (m_index >= m_nodes.size()) {
              m_nodes.resize(m_index + 1);
            }

            m_nodes.at(m_index) = node;
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

    private:
      std::vector<Node> m_nodes;

      KdTreeArray(const KdTreeArray&);
      KdTreeArray& operator=(const KdTreeArray&);
  };
}

#endif /* end of include guard: ARRAY_HPP_BBXOECNY */
