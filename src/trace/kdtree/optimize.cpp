#include "optimize.hpp"

#include <vector>

using namespace std;
namespace trace
{
  namespace kdtree
  {
    namespace
    {
      void set(vector<KdTreeArray::Node>& nodes, unsigned int index, KdTreeArray::Node&& node)
      {
        if (index >= nodes.size()) {
          nodes.resize(index + 1);
        }

        nodes[index] = node;
      }

      void copy(vector<Triangle>& to, const vector<const Triangle*>& from)
      {
        for (const Triangle* tri : from) {
          assert(tri != nullptr);
          to.push_back(*tri);
        }
      }

      void helper(KdTreeArray& result, unsigned int index, KdTreeLinked::Node* node)
      {
        assert(node != nullptr);

        if (is_leaf(*node)) {
          if (has_triangles(*node)) {
            uint32_t i = static_cast<uint32_t>(result.leaf_store.size());
            result.leaf_store.push_back(vector<Triangle>());
            copy(result.leaf_store[i], get_triangles(*node));
            set(result.nodes, index, KdTreeArray::Node(i));
          } else {
            set(result.nodes, index, KdTreeArray::Node());
          }
        } else {
          assert(is_split(*node));

          KdTreeLinked::Node* left  = node->split.left;
          KdTreeLinked::Node* right = node->split.right;

          assert(left != nullptr);
          assert(right != nullptr);

          set(result.nodes, index, KdTreeArray::Node(node->split.distance));

          helper(result, KdTreeArray::left_child(index), left);
          helper(result, KdTreeArray::right_child(index), right);
        }
      }
    }

    void optimize(KdTreeArray& result, const KdTreeLinked& input)
    {
      helper(result, 0, input.root);

      result.leaf_store.shrink_to_fit();
      result.nodes.shrink_to_fit();
    }
  }
}
