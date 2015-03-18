#include "optimize.hpp"

#include <vector>

using namespace std;
namespace trace
{
  namespace kdtree
  {
    namespace
    {
      void helper(KdTreeArray& result, unsigned int index, const LinkedNode* node)
      {
        assert(node != nullptr);

        if (node->is_leaf()) {
          if (node->has_triangles()) {
            uint32_t i = result.store_triangles(node->get_triangles());
            result.set_node(index, ArrayNode(i));
          } else {
            result.set_node(index, ArrayNode());
          }
        } else {
          assert(node->is_split());

          const LinkedNode* left  = node->get_left();
          const LinkedNode* right = node->get_right();

          assert(left != nullptr);
          assert(right != nullptr);

          result.set_node(index, ArrayNode(node->get_split()));

          helper(result, KdTreeArray::left_child(index), left);
          helper(result, KdTreeArray::right_child(index), right);
        }
      }
    }

    void optimize(KdTreeArray& result, const KdTreeLinked& input)
    {
      helper(result, 0, input.get_root());

      result.shrink_to_fit();
    }
  }
}
