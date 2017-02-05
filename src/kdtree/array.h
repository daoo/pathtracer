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
  KdNodeArray() : index_(EMPTY_LEAF) {}
  explicit KdNodeArray(uint32_t i) : index_((i << 1) & MASK_INDEX) {}
  explicit KdNodeArray(float distance) : distance_(distance) {
    index_ |= MASK_TYPE;
  }

  bool is_leaf() const { return (index_ & MASK_TYPE) == TYPE_LEAF; }
  bool is_split() const { return (index_ & MASK_TYPE) == TYPE_SPLIT; }

  uint32_t get_index() const {
    assert(is_leaf());
    return index_ >> 1;
  }

  float get_split() const {
    assert(is_split());
    return distance_;
  }

 private:
  union {
    uint32_t index_;
    float distance_;
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
    assert(index < nodes_.size());
    return nodes_[index];
  }

  const std::vector<geometry::Triangle>& get_triangles(KdNodeArray node) const {
    assert(node.is_leaf());
    return leaf_store_[node.get_index()];
  }

  void add_leaf(unsigned int node_index,
                const std::vector<const geometry::Triangle*>& triangles) {
    std::vector<geometry::Triangle> to;
    for (const geometry::Triangle* tri : triangles) {
      assert(tri != nullptr);
      to.push_back(*tri);
    }

    uint32_t triangles_index = static_cast<uint32_t>(leaf_store_.size());
    leaf_store_.push_back(to);

    if (node_index >= nodes_.size()) {
      nodes_.resize(node_index + 1);
    }

    nodes_[node_index] = KdNodeArray(triangles_index);
  }

  void add_split(unsigned int node_index, float distance) {
    if (node_index >= nodes_.size()) {
      nodes_.resize(node_index + 1);
    }

    nodes_[node_index] = KdNodeArray(distance);
  }

  void shrink_to_fit() {
    nodes_.shrink_to_fit();
    leaf_store_.shrink_to_fit();
  }

  static unsigned int left_child(unsigned int index) {
    return (index << 1) + 1;
  }

  static unsigned int right_child(unsigned int index) {
    return (index << 1) + 2;
  }

 private:
  std::vector<KdNodeArray> nodes_;
  std::vector<std::vector<geometry::Triangle>> leaf_store_;
};
}  // namespace kdtree

#endif  // KDTREE_ARRAY_H_
