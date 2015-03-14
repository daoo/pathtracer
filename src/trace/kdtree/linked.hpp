#ifndef LINKED_HPP_DGVBYSLC
#define LINKED_HPP_DGVBYSLC

#include "trace/geometry/ray.hpp"
#include "trace/geometry/triangle.hpp"
#include "trace/kdtree/util.hpp"

#include <vector>

namespace trace
{
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

    inline bool is_leaf(const KdTreeLinked::Node& node)
    {
      return node.type == KdTreeLinked::Node::Leaf;
    }

    inline bool has_triangles(const KdTreeLinked::Node& node)
    {
      assert(is_leaf(node));
      return node.leaf.triangles != nullptr;
    }

    inline const std::vector<const Triangle*>& get_triangles(
        const KdTreeLinked::Node& node)
    {
      assert(is_leaf(node));
      assert(has_triangles(node));
      return *node.leaf.triangles;
    }

    inline bool is_split(const KdTreeLinked::Node& node)
    {
      return node.type == KdTreeLinked::Node::Split;
    }

    inline Axis get_axis(const KdTreeLinked::Node& node)
    {
      assert(is_split(node));
      return node.split.axis;
    }

    inline float get_split(const KdTreeLinked::Node& node)
    {
      assert(is_split(node));
      return node.split.distance;
    }
  }
}

#endif /* end of include guard: LINKED_HPP_DGVBYSLC */
