#include "kdtree/naive.h"

#include <glm/glm.hpp>

#include <cassert>

#include "geometry/aabb.h"
#include "geometry/bounding.h"
#include "kdtree/util.h"

using glm::vec3;
using std::vector;

namespace geometry {
struct Triangle;
}  // kdtree geometry

namespace kdtree {
namespace {
void go(LinkedNode* node, unsigned int depth, Axis axis, const Box& parent) {
  assert(node != nullptr);

  if (depth >= 20 || parent.triangles.size() <= 10) {
    node->type = LinkedNode::NodeType::Leaf;
    node->leaf.triangles =
        new vector<const geometry::Triangle*>(parent.triangles);
  } else {
    float distance = parent.boundary.center[axis];

    Split split = split_box(parent, axis, distance);
    node->type = LinkedNode::NodeType::Split;
    node->split.axis = axis;
    node->split.distance = distance;
    node->split.left = new LinkedNode;
    node->split.right = new LinkedNode;

    go(node->split.left, depth + 1, next_axis(axis), split.left);
    go(node->split.right, depth + 1, next_axis(axis), split.right);
  }
}
}  // namespace

KdTreeLinked build_tree_naive(const vector<geometry::Triangle>& triangles) {
  vector<const geometry::Triangle*> ptrs;
  for (const geometry::Triangle& tri : triangles) {
    ptrs.push_back(&tri);
  }

  LinkedNode* root = new LinkedNode;
  go(root, 0, X, Box{find_bounding(triangles), ptrs});

  return KdTreeLinked(root);
}
}  // namespace kdtree
