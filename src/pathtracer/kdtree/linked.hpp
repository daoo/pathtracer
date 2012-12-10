#ifndef LINKED_HPP_DGVBYSLC
#define LINKED_HPP_DGVBYSLC

#include "math/ray.hpp"
#include "pathtracer/kdtree/util.hpp"
#include "pathtracer/triangle.hpp"

#include <vector>

namespace kdtree {
  class KdTreeLinked {
    public:
      KdTreeLinked() : m_root(nullptr) { }
      ~KdTreeLinked() { delete m_root; }

      class Node {
        public:
          Node() { }
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

          bool isLeaf() const {
            return m_type == Leaf;
          }

          bool hasTriangles() const {
            assert(isLeaf());
            return m_leaf.m_triangles != nullptr;
          }

          const std::vector<const Triangle*>* triangles() const {
            assert(isLeaf());
            assert(hasTriangles());
            return m_leaf.m_triangles;
          }

          bool isSplit() const {
            return m_type == Node::Split;
          }

          Axis axis() const {
            assert(isSplit());
            return m_split.m_axis;
          }

          float split() const {
            assert(isSplit());
            return m_split.m_distance;
          }
      };

      Node* m_root;

    private:
      KdTreeLinked(const KdTreeLinked&);
      KdTreeLinked& operator=(const KdTreeLinked&);
  };
}

#endif /* end of include guard: LINKED_HPP_DGVBYSLC */
