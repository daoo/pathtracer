#ifndef KDTREE_ARRAY_H_
#define KDTREE_ARRAY_H_

#include <cassert>
#include <cstdint>
#include <limits>
#include <optional>
#include <vector>

#include "geometry/triangle.h"

namespace geometry {
struct Ray;
struct TriRayIntersection;
}  // namespace geometry

namespace kdtree {
struct Intersection;

class KdNodeArray {
 public:
  KdNodeArray() : index_(EMPTY_LEAF) {}
  explicit KdNodeArray(uint32_t i) : index_((i << 1) & MASK_INDEX) {}
  explicit KdNodeArray(float distance) : distance_(distance) {
    index_ |= MASK_TYPE;
  }

  bool IsLeaf() const { return (index_ & MASK_TYPE) == TYPE_LEAF; }
  bool IsSplit() const { return (index_ & MASK_TYPE) == TYPE_SPLIT; }

  uint32_t GetIndex() const {
    assert(IsLeaf());
    return index_ >> 1;
  }

  float GetSplit() const {
    assert(IsSplit());
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
  KdTreeArray(const std::vector<KdNodeArray>& nodes,
              const std::vector<std::vector<geometry::Triangle>>& leaf_store)
      : nodes_(nodes), leaf_store_(leaf_store) {}

  KdNodeArray GetNode(unsigned int index) const {
    assert(index < nodes_.size());
    return nodes_[index];
  }

  const std::vector<geometry::Triangle>& GetTriangles(KdNodeArray node) const {
    assert(node.IsLeaf());
    return leaf_store_[node.GetIndex()];
  }

  static unsigned int LeftChild(unsigned int index) { return (index << 1) + 1; }

  static unsigned int RightChild(unsigned int index) {
    return (index << 1) + 2;
  }

 private:
  std::vector<KdNodeArray> nodes_;
  std::vector<std::vector<geometry::Triangle>> leaf_store_;
};

std::optional<geometry::TriRayIntersection> search_tree(
    const KdTreeArray& tree,
    const geometry::Ray& ray,
    float tmin,
    float tmax);
}  // namespace kdtree

#endif  // KDTREE_ARRAY_H_
