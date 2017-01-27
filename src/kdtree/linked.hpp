#ifndef LINKED_HPP_DGVBYSLC
#define LINKED_HPP_DGVBYSLC

#include "geometry/triangle.hpp"
#include "kdtree/util.hpp"

#include <vector>

namespace trace {
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

  const std::vector<const Triangle*>& get_triangles() const {
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

  // TODO: private
  struct SplitNode {
    Axis axis;
    float distance;

    LinkedNode* left;
    LinkedNode* right;
  };

  struct LeafNode {
    std::vector<const Triangle*>* triangles;
  };

  NodeType type;

  union {
    LeafNode leaf;
    SplitNode split;
  };
};

class KdTreeLinked {
 public:
  KdTreeLinked(const LinkedNode* root) : root(root) { assert(root != nullptr); }

  ~KdTreeLinked() { delete root; }

  const LinkedNode* get_root() const { return root; }

 private:
  const LinkedNode* root;
};
}
}

#endif /* end of include guard: LINKED_HPP_DGVBYSLC */
