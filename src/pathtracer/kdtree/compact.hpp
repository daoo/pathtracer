#ifndef COMPACT_HPP_PJZBFAY0
#define COMPACT_HPP_PJZBFAY0

#include "math/ray.hpp"
#include "pathtracer/kdtree/util.hpp"
#include "pathtracer/triangle.hpp"

#include <vector>

namespace kdtree {
  class KdTreeCompact {
    public:
      KdTreeCompact() : m_root(new Node) { }
      ~KdTreeCompact() { delete m_root; }

      class Node {
        public:
          Node() { }
          ~Node() {
            throw std::string("Not implemented");
          }

        private:
          static constexpr size_t NODE_TYPE_MASK = 0x1; // 0b1

          static constexpr size_t SHIFT_BITS     = 4;
          static constexpr size_t AXIS_MASK      = 0xE; // 0b1110
          static constexpr size_t DISTANCE_MASK  = ~(NODE_TYPE_MASK + AXIS_MASK);
          static constexpr size_t CHILD_PTR_MASK = ~(NODE_TYPE_MASK + AXIS_MASK);

          static constexpr bool is_leaf(int64_t bitfield) {
            return (bitfield & NODE_TYPE_MASK) == 0;
          }

          static constexpr bool is_split(int64_t bitfield) {
            return (bitfield & NODE_TYPE_MASK) == 1;
          }

          static constexpr float distance(int64_t bitfield) {
            return (bitfield & DISTANCE_MASK) >> SHIFT_BITS;
          }

          static Triangle* triangles_ptr(size_t pointer) {
            return reinterpret_cast<Triangle*>(pointer);
          }

          static Node* left_ptr(size_t pointer) {
            return reinterpret_cast<Node*>(pointer);
          }

          struct Leaf {
            int32_t triangles;
          };

          struct Split {
            float d;
          };

          union {
            Leaf m_leaf;
            Split m_split;
          };
      };

      class TraverseIter {
        public:
          TraverseIter(const KdTreeCompact&) { }

          bool isLeaf() const {
            throw std::string("Not implemented");
          }

          bool hasTriangles() const {
            throw std::string("Not implemented");
          }

          bool isSplit() const {
            throw std::string("Not implemented");
          }

          Axis axis() const {
            throw std::string("Not implemented");
          }

          float split() const {
            throw std::string("Not implemented");
          }

          TraverseIter left() const {
            throw std::string("Not implemented");
          }

          TraverseIter right() const {
            throw std::string("Not implemented");
          }

          const std::vector<Triangle>& triangles() const {
            throw std::string("Not implemented");
          }

        private:
          const Node* m_node;

          TraverseIter(Node* n) : m_node(n) {
            assert(n != nullptr);
          }
      };

    private:
      Node* m_root;

      KdTreeCompact(const KdTreeCompact&);
      KdTreeCompact& operator=(const KdTreeCompact&);
  };
}

#endif /* end of include guard: COMPACT_HPP_PJZBFAY0 */
