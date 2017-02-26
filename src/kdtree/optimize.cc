#include "kdtree/optimize.h"

#include <cassert>

#include "kdtree/array.h"
#include "kdtree/linked.h"

namespace kdtree {
namespace {
void helper(KdTreeArray& result, unsigned int index, const KdNodeLinked* node) {
  assert(node != nullptr);

  if (node->GetTriangles() != nullptr) {
    std::vector<geometry::Triangle> copies;
    for (const geometry::Triangle* ptr : *node->GetTriangles()) {
      copies.emplace_back(*ptr);
    }
    result.add_leaf(index, copies);
  } else {
    result.add_split(index, node->GetDistance());

    helper(result, KdTreeArray::left_child(index), node->GetLeft());
    helper(result, KdTreeArray::right_child(index), node->GetRight());
  }
}
}  // namespace

KdTreeArray optimize(const KdTreeLinked& tree) {
  KdTreeArray result;
  helper(result, 0, tree.GetRoot());
  result.shrink_to_fit();
  return result;
}
}  // namespace kdtree
