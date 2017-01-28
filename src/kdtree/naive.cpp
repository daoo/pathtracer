#include "kdtree/naive.hpp"

#include "geometry/aabb.hpp"
#include "geometry/bounding.hpp"
#include "kdtree/util.hpp"
#include <cassert>
#include <glm/glm.hpp>

using namespace glm;
using namespace std;

namespace geometry {
struct Triangle;
}

namespace kdtree {
namespace {
void go(LinkedNode* node,
        unsigned int depth,
        Axis axis,
        const geometry::Aabb& box,
        const vector<const geometry::Triangle*>& triangles) {
  assert(node != nullptr);

  if (depth >= 20 || triangles.size() <= 10) {
    node->type = LinkedNode::NodeType::Leaf;
    node->leaf.triangles = new vector<const geometry::Triangle*>(triangles);
  } else {
    float split = box.center[axis];

    geometry::Aabb left_box;
    geometry::Aabb right_box;

    split_aabb(box, axis, split, left_box, right_box);

    vector<const geometry::Triangle *> left_triangles, right_triangles;

    intersect_test(left_box, right_box, triangles, left_triangles,
                   right_triangles);

    node->type = LinkedNode::NodeType::Split;
    node->split.axis = axis;
    node->split.distance = split;
    node->split.left = new LinkedNode;
    node->split.right = new LinkedNode;

    go(node->split.left, depth + 1, next_axis(axis), left_box, left_triangles);
    go(node->split.right, depth + 1, next_axis(axis), right_box,
       right_triangles);
  }
}
}

KdTreeLinked build_tree_naive(const vector<geometry::Triangle>& triangles) {
  vector<const geometry::Triangle*> ptrs;
  for (const geometry::Triangle& tri : triangles) {
    ptrs.push_back(&tri);
  }

  LinkedNode* root = new LinkedNode;
  go(root, 0, X, find_bounding(triangles), ptrs);

  return KdTreeLinked(root);
}
}
