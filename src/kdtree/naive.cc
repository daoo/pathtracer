#include "kdtree/naive.h"

#include <glm/glm.hpp>

#include "geometry/aabb.h"
#include "geometry/bounding.h"
#include "kdtree/linked.h"
#include "kdtree/util.h"

using glm::vec3;
using std::vector;

namespace geometry {
struct Triangle;
}  // kdtree geometry

namespace kdtree {
namespace {
KdNodeLinked* go(unsigned int depth, Axis axis, const Box& parent) {
  if (depth >= 20 || parent.triangles.size() <= 6) {
    return new KdNodeLinked(
        new vector<const geometry::Triangle*>(parent.triangles));
  } else {
    float distance = parent.boundary.GetCenter()[axis];
    Split split = split_box(parent, axis, distance);
    return new KdNodeLinked(axis, distance,
                            go(depth + 1, next_axis(axis), split.left),
                            go(depth + 1, next_axis(axis), split.right));
  }
}
}  // namespace

KdTreeLinked build_tree_naive(const vector<geometry::Triangle>& triangles) {
  vector<const geometry::Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const geometry::Triangle& tri : triangles) {
    ptrs.emplace_back(&tri);
  }

  return {go(0, X, Box{find_bounding(triangles), ptrs}), triangles};
}
}  // namespace kdtree
