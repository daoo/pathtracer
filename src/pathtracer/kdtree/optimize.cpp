#include "optimize.hpp"

#include <vector>

using namespace std;

namespace kdtree {
  namespace {
    void set(vector<KdTreeArray::Node>& nodes, size_t index, KdTreeArray::Node&& node) {
      if (index >= nodes.size()) {
        nodes.resize(index + 1);
      }

      nodes[index] = node;
    }

    void copy(vector<Triangle>& to, const vector<const Triangle*>& from) {
      for (const Triangle* tri : from) {
        to.push_back(*tri);
      }
    }

    void helper(KdTreeArray& result, size_t index, KdTreeLinked::Node* node) {
      assert(node != nullptr);

      if (node->isLeaf()) {
        if (node->hasTriangles()) {
          uint32_t i = static_cast<uint32_t>(result.m_leaf_store.size());
          result.m_leaf_store.push_back(vector<Triangle>());
          copy(result.m_leaf_store[i], *node->triangles());
          set(result.m_nodes, index, KdTreeArray::Node(i));
        } else {
          set(result.m_nodes, index, KdTreeArray::Node());
        }
      } else if (node->isSplit()) {
        KdTreeLinked::Node* left  = node->m_split.m_left;
        KdTreeLinked::Node* right = node->m_split.m_right;

        assert(left != nullptr);
        assert(right != nullptr);

        bool equalChildren = true;
        if (left->isLeaf() && right->isLeaf() &&
            left->hasTriangles() && right->hasTriangles() &&
            left->triangles()->size() == right->triangles()->size()) {
          auto itl = left->triangles()->cbegin();
          auto itr = right->triangles()->cbegin();

          while (itl < left->triangles()->cend()) {
            if (*itl != *itr) {
              equalChildren = false;
              break;
            }
            ++itl;
            ++itr;
          }
        } else {
          equalChildren = false;
        }

        if (equalChildren) {
          uint32_t i = static_cast<uint32_t>(result.m_leaf_store.size());
          result.m_leaf_store.push_back(vector<Triangle>());
          copy(result.m_leaf_store[i], *node->triangles());
          set(result.m_nodes, index, KdTreeArray::Node(i));
        } else {
          set(result.m_nodes, index, KdTreeArray::Node(node->m_split.m_distance));
        }

        helper(result, KdTreeArray::leftChild(index), left);
        helper(result, KdTreeArray::rightChild(index), right);
      }
    }
  }

  void optimize(KdTreeArray& result, const KdTreeLinked& input) {
    helper(result, 0, input.m_root);
  }
}
