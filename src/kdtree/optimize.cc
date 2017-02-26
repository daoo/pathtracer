#include "kdtree/optimize.h"

#include <cassert>
#include <cstdint>
#include <vector>

#include "geometry/triangle.h"
#include "kdtree/array.h"
#include "kdtree/linked.h"

namespace kdtree {
namespace {
void helper(unsigned int index,
            const KdNodeLinked* node,
            std::vector<KdNodeArray>& nodes,
            std::vector<std::vector<geometry::Triangle>>& leaf_store) {
  assert(node != nullptr);

  if (node->GetTriangles() != nullptr) {
    std::vector<geometry::Triangle> copies;
    for (const geometry::Triangle* ptr : *node->GetTriangles()) {
      copies.emplace_back(*ptr);
    }
    uint32_t triangles_index = static_cast<uint32_t>(leaf_store.size());
    leaf_store.emplace_back(copies);
    if (index >= nodes.size()) {
      nodes.resize(index + 1);
    }
    nodes[index] = KdNodeArray(triangles_index);
  } else {
    if (index >= nodes.size()) {
      nodes.resize(index + 1);
    }
    nodes[index] = KdNodeArray(node->GetDistance());

    helper(KdTreeArray::left_child(index), node->GetLeft(), nodes, leaf_store);
    helper(KdTreeArray::right_child(index), node->GetRight(), nodes,
           leaf_store);
  }
}
}  // namespace

KdTreeArray optimize(const KdTreeLinked& tree) {
  std::vector<KdNodeArray> nodes;
  std::vector<std::vector<geometry::Triangle>> leaf_store;
  helper(0, tree.GetRoot(), nodes, leaf_store);
  nodes.shrink_to_fit();
  leaf_store.shrink_to_fit();
  return {nodes, leaf_store};
}
}  // namespace kdtree
