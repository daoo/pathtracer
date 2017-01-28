#include "optimize.hpp"

#include "kdtree/array.hpp"
#include <cassert>

using namespace std;
namespace kdtree {
namespace {
void helper(KdTreeArray& result, unsigned int index, const LinkedNode* node) {
  assert(node != nullptr);

  if (node->is_leaf()) {
    result.add_leaf(index, node->get_triangles());
  } else {
    assert(node->is_split());

    result.add_split(index, node->get_split());

    helper(result, KdTreeArray::left_child(index), node->get_left());
    helper(result, KdTreeArray::right_child(index), node->get_right());
  }
}
}

void optimize(KdTreeArray& result, const KdTreeLinked& input) {
  helper(result, 0, input.get_root());
  result.shrink_to_fit();
}
}
