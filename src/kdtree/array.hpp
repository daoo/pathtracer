#ifndef ARRAY_HPP_BBXOECNY
#define ARRAY_HPP_BBXOECNY

#include "geometry/triangle.hpp"
#include "kdtree/util.hpp"

#include <algorithm>
#include <cstdint>
#include <limits>
#include <vector>

namespace trace {
namespace kdtree {
class ArrayNode {
 public:
  ArrayNode() : index(EMPTY_LEAF) {}
  ArrayNode(uint32_t i) : index((i << 1) & MASK_INDEX) {}
  ArrayNode(float distance) : distance(distance) { index |= MASK_TYPE; }

  bool is_leaf() const { return (index & MASK_TYPE) == TYPE_LEAF; }
  bool is_split() const { return (index & MASK_TYPE) == TYPE_SPLIT; }

  uint32_t get_index() const {
    assert(is_leaf());
    return index >> 1;
  }

  float get_split() const {
    assert(is_split());
    return distance;
  }

 private:
  enum NodeType { Split, Leaf };

  union {
    uint32_t index;
    float distance;
  };

  static constexpr uint32_t MASK_TYPE = 0x1;
  static constexpr uint32_t MASK_INDEX = ~MASK_TYPE;

  static constexpr uint32_t EMPTY_LEAF =
      std::numeric_limits<uint32_t>::max() & MASK_INDEX;

  static constexpr uint32_t TYPE_LEAF = 0;
  static constexpr uint32_t TYPE_SPLIT = 1;
};

static_assert(sizeof(ArrayNode) == 4, "incorrect size");

class KdTreeArray {
 public:
  ArrayNode get_node(unsigned int index) const {
    assert(index < nodes.size());
    return nodes[index];
  }

  const std::vector<Triangle>& get_triangles(ArrayNode node) const {
    assert(node.is_leaf());
    return leaf_store[node.get_index()];
  }

  void add_leaf(unsigned int node_index,
                const std::vector<const Triangle*>& triangles) {
    uint32_t triangles_index = static_cast<uint32_t>(leaf_store.size());
    leaf_store.push_back(std::vector<Triangle>());

    std::vector<Triangle>& to = leaf_store[triangles_index];
    for (const Triangle* tri : triangles) {
      assert(tri != nullptr);
      to.push_back(*tri);
    }

    if (node_index >= nodes.size()) {
      nodes.resize(node_index + 1);
    }

    nodes[node_index] = ArrayNode(triangles_index);
  }

  void add_split(unsigned int node_index, float distance) {
    if (node_index >= nodes.size()) {
      nodes.resize(node_index + 1);
    }

    nodes[node_index] = ArrayNode(distance);
  }

  void shrink_to_fit() {
    nodes.shrink_to_fit();
    leaf_store.shrink_to_fit();
  }

  static unsigned int left_child(unsigned int index) {
    return (index << 1) + 1;
  }

  static unsigned int right_child(unsigned int index) {
    return (index << 1) + 2;
  }

 private:
  std::vector<ArrayNode> nodes;
  std::vector<std::vector<Triangle>> leaf_store;
};
}
}

#endif /* end of include guard: ARRAY_HPP_BBXOECNY */
