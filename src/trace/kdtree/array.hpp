#ifndef ARRAY_HPP_BBXOECNY
#define ARRAY_HPP_BBXOECNY

#include "trace/geometry/ray.hpp"
#include "trace/geometry/triangle.hpp"
#include "trace/kdtree/util.hpp"

#include <array>
#include <cstdint>
#include <limits>
#include <vector>

namespace trace
{
  namespace kdtree
  {
    struct KdTreeArray
    {
      struct Node
      {
        Node() : index(EMPTY_LEAF) { }
        Node(uint32_t i) : index((i << 1) & MASK_INDEX) { }
        Node(float distance) : distance(distance) { index |= MASK_TYPE; }

        enum NodeType { Split, Leaf };

        union
        {
          uint32_t index;
          float distance;
        };

        static constexpr uint32_t MASK_TYPE  = 0x1;
        static constexpr uint32_t MASK_INDEX = ~MASK_TYPE;

        static constexpr uint32_t EMPTY_LEAF =
          std::numeric_limits<uint32_t>::max() & MASK_INDEX;

        static constexpr uint32_t TYPE_LEAF = 0;
        static constexpr uint32_t TYPE_SPLIT = 1;
      };

      static unsigned int left_child(unsigned int index)
      {
        return (index << 1) + 1;
      }

      static unsigned int right_child(unsigned int index)
      {
        return (index << 1) + 2;
      }

      std::vector<Node> nodes;
      std::vector<std::vector<Triangle>> leaf_store;
    };

    bool is_leaf(const KdTreeArray::Node& node);
    bool has_triangles(const KdTreeArray::Node& node);
    const std::vector<Triangle>& get_triangles(
        const KdTreeArray& tree,
        const KdTreeArray::Node& node);
    bool is_split(const KdTreeArray::Node& node);
    uint32_t get_index(const KdTreeArray::Node& node);
    float get_split(const KdTreeArray::Node& node);

    inline bool is_leaf(const KdTreeArray::Node& node)
    {
      return (node.index & KdTreeArray::Node::MASK_TYPE) ==
        KdTreeArray::Node::TYPE_LEAF;
    }

    inline bool has_triangles(const KdTreeArray::Node& node)
    {
      assert(is_leaf(node));
      return node.index != KdTreeArray::Node::EMPTY_LEAF;
    }

    inline uint32_t get_index(const KdTreeArray::Node& node)
    {
      assert(is_leaf(node));
      return node.index >> 1;
    }

    inline const std::vector<Triangle>& get_triangles(
        const KdTreeArray& tree,
        const KdTreeArray::Node& node)
    {
      assert(is_leaf(node));
      assert(has_triangles(node));
      return tree.leaf_store[get_index(node)];
    }

    inline bool is_split(const KdTreeArray::Node& node)
    {
      return (node.index & KdTreeArray::Node::MASK_TYPE) ==
        KdTreeArray::Node::TYPE_SPLIT;
    }

    inline float get_split(const KdTreeArray::Node& node)
    {
      assert(is_split(node));
      return node.distance;
    }
  }
}

#endif /* end of include guard: ARRAY_HPP_BBXOECNY */
