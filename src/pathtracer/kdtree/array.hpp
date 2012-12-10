#ifndef ARRAY_HPP_BBXOECNY
#define ARRAY_HPP_BBXOECNY

#include "math/ray.hpp"
#include "pathtracer/kdtree/util.hpp"
#include "pathtracer/triangle.hpp"

#include <array>
#include <cstdint>
#include <limits>
#include <vector>

namespace kdtree {
  class KdTreeArray {
    public:
      KdTreeArray() : m_nodes(), m_leaf_store() { }
      ~KdTreeArray() { }

      class Node {
        public:
          enum NodeType { Split, Leaf };

          ~Node() { }

          Node() : m_data(EMPTY_LEAF) { }
          Node(uint32_t index) : m_data((index << 1) & MASK_DATA) { }
          Node(float distance) : m_distance(distance) { m_data |= MASK_TYPE; }

          bool isLeaf() const { return (m_data & MASK_TYPE) == TYPE_LEAF; }
          bool isSplit() const { return (m_data & MASK_TYPE) == TYPE_SPLIT; }

          uint32_t getIndex() const { return m_data >> 1; }
          float getDistance() const { return m_distance; }

          bool hasTriangles() const { return m_data != EMPTY_LEAF; }

        private:
          union {
            uint32_t m_data;
            float m_distance;
          };

          static constexpr uint32_t MASK_TYPE = 0x1;
          static constexpr uint32_t MASK_DATA = ~MASK_TYPE;

          static constexpr uint32_t EMPTY_LEAF = std::numeric_limits<uint32_t>::max() & MASK_DATA;

          static constexpr uint32_t TYPE_LEAF = 0;
          static constexpr uint32_t TYPE_SPLIT = 1;
      };

      class TraverseIter {
        public:
          TraverseIter(const KdTreeArray& tree) :
            m_tree(tree), m_node(&tree.m_nodes[0]), m_index(0),
            m_axis(X) { }

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
            return m_node->isLeaf();
          }

          bool hasTriangles() const {
            assert(isLeaf());
            return m_node->hasTriangles();
          }

          bool isSplit() const {
            return m_node->isSplit();
          }

          Axis axis() const {
            assert(isSplit());
            return m_axis;
          }

          float split() const {
            assert(isSplit());
            return m_node->getDistance();
          }

          TraverseIter left() const {
            assert(isSplit());
            return TraverseIter(m_tree, leftChild(m_index), next(m_axis));
          }

          TraverseIter right() const {
            assert(isSplit());
            return TraverseIter(m_tree, rightChild(m_index), next(m_axis));
          }

          const std::vector<Triangle>& triangles() const {
            assert(isLeaf() && hasTriangles());
            return m_tree.m_leaf_store[m_node->getIndex()];
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

      static Axis next(Axis axis) {
        return static_cast<Axis>((static_cast<size_t>(axis) + 1) % 3);
      }

      static size_t leftChild(size_t index) {
        return (index << 1) + 1;
      }

      static size_t rightChild(size_t index) {
        return (index << 1) + 2;
      }

      std::vector<Node> m_nodes;
      std::vector<std::vector<Triangle>> m_leaf_store;

    private:
      KdTreeArray(const KdTreeArray&);
      KdTreeArray& operator=(const KdTreeArray&);
  };
}

#endif /* end of include guard: ARRAY_HPP_BBXOECNY */
