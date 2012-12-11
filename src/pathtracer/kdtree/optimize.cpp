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
        assert(tri != nullptr);
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
      } else {
        assert(node->isSplit());

        KdTreeLinked::Node* left  = node->m_split.m_left;
        KdTreeLinked::Node* right = node->m_split.m_right;

        assert(left != nullptr);
        assert(right != nullptr);

        set(result.m_nodes, index, KdTreeArray::Node(node->m_split.m_distance));

        helper(result, KdTreeArray::leftChild(index), left);
        helper(result, KdTreeArray::rightChild(index), right);
      }
    }
  }

  void optimize(KdTreeArray& result, const KdTreeLinked& input) {
    helper(result, 0, input.m_root);
  }
}
