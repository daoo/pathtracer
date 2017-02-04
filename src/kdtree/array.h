#ifndef KDTREE_ARRAY_H_
#define KDTREE_ARRAY_H_

#include <algorithm>
#include <cstdint>
#include <limits>
#include <vector>

#include "geometry/triangle.h"
#include "kdtree/util.h"

namespace kdtree {
class KdNodeArray {
 public:
  KdNodeArray() : index(EMPTY_LEAF) {}
  explicit KdNodeArray(uint32_t i) : index((i << 1) & MASK_INDEX) {}
  explicit KdNodeArray(float distance) : distance(distance) {
    index |= MASK_TYPE;
  }

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

static_assert(sizeof(KdNodeArray) == 4, "incorrect size");

class KdTreeArray {
 public:
  KdNodeArray get_node(unsigned int index) const {
    assert(index < nodes.size());
    return nodes[index];
  }

  const std::vector<geometry::Triangle>& get_triangles(KdNodeArray node) const {
    assert(node.is_leaf());
    return leaf_store[node.get_index()];
  }

  void add_leaf(unsigned int node_index,
                const std::vector<const geometry::Triangle*>& triangles) {
    std::vector<geometry::Triangle> to;
    for (const geometry::Triangle* tri : triangles) {
      assert(tri != nullptr);
      to.push_back(*tri);
    }

    uint32_t triangles_index = static_cast<uint32_t>(leaf_store.size());
    leaf_store.push_back(to);

    if (node_index >= nodes.size()) {
      nodes.resize(node_index + 1);
    }

    nodes[node_index] = KdNodeArray(triangles_index);
  }

  void add_split(unsigned int node_index, float distance) {
    if (node_index >= nodes.size()) {
      nodes.resize(node_index + 1);
    }

    nodes[node_index] = KdNodeArray(distance);
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
  std::vector<KdNodeArray> nodes;
  std::vector<std::vector<geometry::Triangle>> leaf_store;
};
}  // namespace kdtree

#endif  // KDTREE_ARRAY_H_
