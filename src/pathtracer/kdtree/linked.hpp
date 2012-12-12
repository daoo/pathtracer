#ifndef LINKED_HPP_DGVBYSLC
#define LINKED_HPP_DGVBYSLC

#include "math/ray.hpp"
#include "pathtracer/kdtree/util.hpp"
#include "pathtracer/triangle.hpp"

#include <vector>

namespace kdtree {
  struct KdTreeLinked {
    KdTreeLinked() : m_root(nullptr) { }
    ~KdTreeLinked() { delete m_root; }

    struct Node {
      ~Node() {
        if (m_type == Leaf) {
          delete m_leaf.m_triangles;
        } else if (m_type == Split) {
          delete m_split.m_left;
          delete m_split.m_right;
        }
      }

      enum NodeType { Split, Leaf };

      struct SplitNode {
        Axis m_axis;
        float m_distance;

        Node* m_left;
        Node* m_right;
      };

      struct LeafNode {
        std::vector<const Triangle*>* m_triangles;
      };

      NodeType m_type;

      union {
        LeafNode m_leaf;
        SplitNode m_split;
      };
    };

    Node* m_root;
  };

  inline bool isLeaf(const KdTreeLinked::Node& node) {
    return node.m_type == KdTreeLinked::Node::Leaf;
  }

  inline bool hasTriangles(const KdTreeLinked::Node& node) {
    assert(isLeaf(node));
    return node.m_leaf.m_triangles != nullptr;
  }

  inline const std::vector<const Triangle*>& getTriangles(
      const KdTreeLinked::Node& node) {
    assert(isLeaf(node));
    assert(hasTriangles(node));
    return *node.m_leaf.m_triangles;
  }

  inline bool isSplit(const KdTreeLinked::Node& node) {
    return node.m_type == KdTreeLinked::Node::Split;
  }

  inline Axis getAxis(const KdTreeLinked::Node& node) {
    assert(isSplit(node));
    return node.m_split.m_axis;
  }

  inline float getSplit(const KdTreeLinked::Node& node) {
    assert(isSplit(node));
    return node.m_split.m_distance;
  }
}

#endif /* end of include guard: LINKED_HPP_DGVBYSLC */
