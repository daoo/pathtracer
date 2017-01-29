#ifndef KDTREE_LINKED_H_
#define KDTREE_LINKED_H_

#include <vector>

#include "geometry/triangle.h"
#include "kdtree/util.h"

namespace kdtree {
class LinkedNode {
 public:
  enum NodeType { Split, Leaf };

  ~LinkedNode() {
    if (type == Leaf) {
      delete leaf.triangles;
    } else if (type == Split) {
      delete split.left;
      delete split.right;
    }
  }

  bool is_leaf() const { return type == NodeType::Leaf; }
  bool is_split() const { return type == NodeType::Split; }

  bool has_triangles() const {
    assert(is_leaf());
    return leaf.triangles != nullptr;
  }

  const std::vector<const geometry::Triangle*>& get_triangles() const {
    assert(is_leaf());
    assert(has_triangles());
    return *leaf.triangles;
  }

  Axis get_axis() const {
    assert(is_split());
    return split.axis;
  }

  float get_split() const {
    assert(is_split());
    return split.distance;
  }

  const LinkedNode* get_left() const {
    assert(is_split());
    return split.left;
  }

  const LinkedNode* get_right() const {
    assert(is_split());
    return split.right;
  }

  struct SplitNode {
    Axis axis;
    float distance;

    LinkedNode* left;
    LinkedNode* right;
  };

  struct LeafNode {
    std::vector<const geometry::Triangle*>* triangles;
  };

  NodeType type;

  union {
    LeafNode leaf;
    SplitNode split;
  };
};

class KdTreeLinked {
 public:
  explicit KdTreeLinked(const LinkedNode* root_) : root(root_) {
    assert(root_ != nullptr);
  }
  KdTreeLinked(KdTreeLinked&& other) {
    root = other.root;
    other.root = nullptr;
  }

  ~KdTreeLinked() { delete root; }

  const LinkedNode* get_root() const { return root; }

 private:
  KdTreeLinked(const KdTreeLinked&) = delete;
  KdTreeLinked& operator=(const KdTreeLinked&) = delete;

  const LinkedNode* root;
};
}  // namespace kdtree

#endif  // KDTREE_LINKED_H_
