#include "kdtree/naive.h"

#include <glm/glm.hpp>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "kdtree/linked.h"
#include "kdtree/util.h"

namespace geometry {
struct Triangle;
}  // kdtree geometry

using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using glm::vec3;
using std::vector;

namespace kdtree {
namespace {
KdNodeLinked* go(unsigned int depth, Axis axis, const Box& parent) {
  if (depth >= 20 || parent.triangles.size() <= 6) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  } else {
    Aap plane(axis, parent.boundary.GetCenter()[axis]);
    Split split = split_box(parent, plane);
    KdNodeLinked* left_child = go(depth + 1, next_axis(axis), split.left);
    KdNodeLinked* right_child = go(depth + 1, next_axis(axis), split.right);
    return new KdNodeLinked(plane, left_child, right_child);
  }
}
}  // namespace

KdTreeLinked build_tree_naive(const vector<Triangle>& triangles) {
  vector<const Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  return KdTreeLinked(go(0, geometry::X, Box{find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
