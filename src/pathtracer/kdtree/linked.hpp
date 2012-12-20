#ifndef LINKED_HPP_DGVBYSLC
#define LINKED_HPP_DGVBYSLC

#include "math/ray.hpp"
#include "pathtracer/kdtree/util.hpp"
#include "pathtracer/triangle.hpp"

#include <vector>

namespace kdtree
{
  struct KdTreeLinked
  {
    KdTreeLinked() : root(nullptr) { }
    ~KdTreeLinked() { delete root; }

    struct Node
    {
      ~Node()
      {
        if (type == Leaf) {
          delete leaf.triangles;
        } else if (type == Split) {
          delete split.left;
          delete split.right;
        }
      }

      enum NodeType { Split, Leaf };

      struct SplitNode
      {
        Axis axis;
        float distance;

        Node* left;
        Node* right;
      };

      struct LeafNode
      {
        std::vector<const Triangle*>* triangles;
      };

      NodeType type;

      union
      {
        LeafNode leaf;
        SplitNode split;
      };
    };

    Node* root;
  };

  inline bool isLeaf(const KdTreeLinked::Node& node)
  {
    return node.type == KdTreeLinked::Node::Leaf;
  }

  inline bool hasTriangles(const KdTreeLinked::Node& node)
  {
    assert(isLeaf(node));
    return node.leaf.triangles != nullptr;
  }

  inline const std::vector<const Triangle*>& getTriangles(
      const KdTreeLinked::Node& node)
  {
    assert(isLeaf(node));
    assert(hasTriangles(node));
    return *node.leaf.triangles;
  }

  inline bool isSplit(const KdTreeLinked::Node& node)
  {
    return node.type == KdTreeLinked::Node::Split;
  }

  inline Axis getAxis(const KdTreeLinked::Node& node)
  {
    assert(isSplit(node));
    return node.split.axis;
  }

  inline float getSplit(const KdTreeLinked::Node& node)
  {
    assert(isSplit(node));
    return node.split.distance;
  }
}

#endif /* end of include guard: LINKED_HPP_DGVBYSLC */
