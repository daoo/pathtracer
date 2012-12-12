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
  struct KdTreeArray {
    struct Node {
      Node() : m_data(EMPTY_LEAF) { }
      Node(uint32_t index) : m_data((index << 1) & MASK_DATA) { }
      Node(float distance) : m_distance(distance) { m_data |= MASK_TYPE; }

      enum NodeType { Split, Leaf };

      union {
        uint32_t m_data;
        float m_distance;
      };

      static constexpr uint32_t MASK_TYPE = 0x1;
      static constexpr uint32_t MASK_DATA = ~MASK_TYPE;

      static constexpr uint32_t EMPTY_LEAF =
        std::numeric_limits<uint32_t>::max() & MASK_DATA;

      static constexpr uint32_t TYPE_LEAF = 0;
      static constexpr uint32_t TYPE_SPLIT = 1;
    };

    static size_t leftChild(size_t index) {
      return (index << 1) + 1;
    }

    static size_t rightChild(size_t index) {
      return (index << 1) + 2;
    }

    std::vector<Node> m_nodes;
    std::vector<std::vector<Triangle>> m_leaf_store;
  };

  bool isLeaf(const KdTreeArray::Node& node);
  bool hasTriangles(const KdTreeArray::Node& node);
  const std::vector<Triangle>& getTriangles(
      const KdTreeArray& tree, const KdTreeArray::Node& node);
  bool isSplit(const KdTreeArray::Node& node);
  uint32_t getIndex(const KdTreeArray::Node& node);
  float getSplit(const KdTreeArray::Node& node);


  inline bool isLeaf(const KdTreeArray::Node& node) {
    return (node.m_data & KdTreeArray::Node::MASK_TYPE) ==
      KdTreeArray::Node::TYPE_LEAF;
  }

  inline bool hasTriangles(const KdTreeArray::Node& node) {
    assert(isLeaf(node));
    return node.m_data != KdTreeArray::Node::EMPTY_LEAF;
  }

  inline uint32_t getIndex(const KdTreeArray::Node& node) {
    assert(isLeaf(node));
    return node.m_data >> 1;
  }

  inline const std::vector<Triangle>& getTriangles(
      const KdTreeArray& tree, const KdTreeArray::Node& node) {
    assert(isLeaf(node));
    return tree.m_leaf_store[getIndex(node)];
  }

  inline bool isSplit(const KdTreeArray::Node& node) {
    return (node.m_data & KdTreeArray::Node::MASK_TYPE) ==
      KdTreeArray::Node::TYPE_SPLIT;
  }

  inline float getSplit(const KdTreeArray::Node& node) {
    assert(isSplit(node));
    return node.m_distance;
  }
}

#endif /* end of include guard: ARRAY_HPP_BBXOECNY */
