#include "kdtree/optimize.h"

#include <cassert>

#include "kdtree/array.h"
#include "kdtree/linked.h"

namespace kdtree {
namespace {
void helper(KdTreeArray& result, unsigned int index, const KdNodeLinked* node) {
  assert(node != nullptr);

  if (node->GetTriangles() != nullptr) {
    result.add_leaf(index, *node->GetTriangles());
  } else {
    result.add_split(index, node->GetDistance());

    helper(result, KdTreeArray::left_child(index), node->GetLeft());
    helper(result, KdTreeArray::right_child(index), node->GetRight());
  }
}
}  // namespace

KdTreeArray optimize(const KdNodeLinked* root) {
  KdTreeArray result;
  helper(result, 0, root);
  result.shrink_to_fit();
  return result;
}
}  // namespace kdtree
