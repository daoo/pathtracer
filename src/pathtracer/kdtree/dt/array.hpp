#ifndef ARRAY_HPP_BBXOECNY
#define ARRAY_HPP_BBXOECNY

#include "pathtracer/kdtree/util.hpp"
#include "pathtracer/math/ray.hpp"
#include "pathtracer/triangle.hpp"

#include <array>
#include <limits>
#include <vector>

namespace kdtree {
  class KdTreeArray {
    public:
      KdTreeArray() : m_nodes(), m_leaf_store(), m_starting_axis(X) { }
      ~KdTreeArray() { }

      class Node {
        public:
          enum NodeType { Split, Leaf };

          Node() { }
          ~Node() { }

          Node(int32_t index) : m_type(Leaf), m_triangles(index) { }
          Node(float distance) : m_type(Split), m_distance(distance) { }

          NodeType m_type;

          union {
            int32_t m_triangles;
            float m_distance;
          };
      };

      class BuildIter {
        public:
          BuildIter(KdTreeArray& tree) :
            m_tree(tree), m_index(0), m_depth(0), m_axis(X) { }

          Axis axis() const { return m_axis; }
          size_t depth() const { return m_depth; }

          /**
           * Create a split node.
           */
          void split(float d) {
            setNode(Node({d}));
          }

          /**
           * Create a leaf node.
           */
          void leaf(const std::vector<Triangle>& triangles) {
            int32_t index = m_tree.m_leaf_store.size();

            if (triangles.empty()) {
              index = std::numeric_limits<int32_t>::max();
            } else {
              m_tree.m_leaf_store.push_back(std::vector<Triangle>(triangles));
            }

            setNode(Node(index));
          }

          BuildIter left()  { return BuildIter(m_tree, leftChild(m_index), m_depth + 1, next(m_axis)); }
          BuildIter right() { return BuildIter(m_tree, rightChild(m_index), m_depth + 1, next(m_axis)); }

        private:
          KdTreeArray& m_tree;

          size_t m_index;
          size_t m_depth;
          Axis m_axis;

          BuildIter(KdTreeArray& tree, size_t index, size_t depth, Axis axis) :
              m_tree(tree), m_index(index), m_depth(depth), m_axis(axis) { }

          /**
           * Set the current node.
           */
          void setNode(Node&& node) {
            // When a build iter is created for some node, that node does not
            // acctually exists in the underlying vector.

            if (m_index >= m_tree.m_nodes.size()) {
              m_tree.m_nodes.resize(m_index + 1);
            }

            m_tree.m_nodes.at(m_index) = node;
          }
      };

      class TraverseIter {
        public:
          TraverseIter(const KdTreeArray& tree) :
            m_tree(tree), m_node(&tree.m_nodes[0]), m_index(0),
            m_axis(tree.m_starting_axis) { }

          TraverseIter& operator=(const TraverseIter& iter) {
            assert(this != &iter);

            // We do not allow an iterator to change the tree it iterates
            assert(&m_tree == &iter.m_tree);

            m_node  = iter.m_node;
            m_index = iter.m_index;
            m_axis  = iter.m_axis;

            return *this;
          }

          bool isLeaf() const {
            return m_node->m_type == Node::Leaf;
          }

          bool hasTriangles() const {
            assert(isLeaf());
            return m_node->m_triangles != std::numeric_limits<int32_t>::max();
          }

          bool isSplit() const {
            return m_node->m_type == Node::Split;
          }

          Axis axis() const {
            assert(isSplit());
            return m_axis;
          }

          float split() const {
            assert(isSplit());
            return m_node->m_distance;
          }

          TraverseIter left() const {
            assert(isSplit());
            return TraverseIter(m_tree, (m_index << 1) + 1, next(m_axis));
          }

          TraverseIter right() const {
            assert(isSplit());
            return TraverseIter(m_tree, (m_index << 1) + 2, next(m_axis));
          }

          const std::vector<Triangle>& triangles() const {
            assert(isLeaf() && hasTriangles());
            return m_tree.m_leaf_store[m_node->m_triangles];
          }

        private:
          const KdTreeArray& m_tree;
          const Node* m_node;
          size_t m_index;
          Axis m_axis;

          TraverseIter(const KdTreeArray& tree, size_t index, Axis axis) :
              m_tree(tree), m_node(&m_tree.m_nodes[index]), m_index(index),
              m_axis(axis) { }
      };

    private:
      std::vector<Node> m_nodes;
      std::vector<std::vector<Triangle>> m_leaf_store;
      Axis m_starting_axis;

      KdTreeArray(const KdTreeArray&);
      KdTreeArray& operator=(const KdTreeArray&);

      static Axis next(Axis axis) {
        return static_cast<Axis>((static_cast<size_t>(axis) + 1) % 3);
      }

      static size_t leftChild(size_t index) {
        return (index << 1) + 1;
      }

      static size_t rightChild(size_t index) {
        return (index << 1) + 2;
      }
  };
}

#endif /* end of include guard: ARRAY_HPP_BBXOECNY */
