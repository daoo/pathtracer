#include "kdtree/naive.h"

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>
#include <vector>

#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "kdtree/build_common.h"
#include "kdtree/kdtree.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

using geometry::AabbSplit;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using glm::vec3;
using kdtree::KdBox;
using kdtree::KdNode;
using kdtree::KdSplit;
using std::vector;

namespace {

// A look up table have been empirically proven to be the fastest way to
// calculate the next axis, compared to using modulo addition and bit hacks.
constexpr geometry::Axis NEXT[] = {geometry::Y, geometry::Z, geometry::X};
constexpr inline geometry::Axis next_axis(geometry::Axis axis) {
  return NEXT[axis];
}

KdNode* BuildHelper(unsigned int depth, Axis axis, const KdBox& parent) {
  if (depth >= 20 || parent.triangles.size() <= 6) {
    return new KdNode(new vector<const Triangle*>(parent.triangles));
  } else {
    Aap plane(axis, parent.boundary.GetCenter()[axis]);
    KdSplit split = kdtree::Split(parent, plane, kdtree::LEFT);
    KdNode* left_child = BuildHelper(depth + 1, next_axis(axis), split.left);
    KdNode* right_child = BuildHelper(depth + 1, next_axis(axis), split.right);
    return new KdNode(plane, left_child, right_child);
  }
}

}  // namespace

namespace kdtree {
KdTree build(const vector<Triangle>& triangles) {
  vector<const Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  return KdTree(BuildHelper(0, geometry::X,
                            KdBox{geometry::find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
