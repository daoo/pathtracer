#include "kdtree/naive.h"

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>
#include <vector>

#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "kdtree/axis.h"
#include "kdtree/build_common.h"
#include "kdtree/linked.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

using geometry::AabbSplit;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using glm::vec3;
using kdtree::KdBox;
using kdtree::KdNodeLinked;
using kdtree::KdSplit;
using std::vector;

namespace {

KdNodeLinked* BuildHelper(unsigned int depth, Axis axis, const KdBox& parent) {
  if (depth >= 20 || parent.triangles.size() <= 6) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  } else {
    Aap plane(axis, parent.boundary.GetCenter()[axis]);
    KdSplit split = kdtree::Split(parent, plane, kdtree::LEFT);
    KdNodeLinked* left_child =
        BuildHelper(depth + 1, kdtree::next_axis(axis), split.left);
    KdNodeLinked* right_child =
        BuildHelper(depth + 1, kdtree::next_axis(axis), split.right);
    return new KdNodeLinked(plane, left_child, right_child);
  }
}

}  // namespace

namespace kdtree {
KdTreeLinked build(const vector<Triangle>& triangles) {
  vector<const Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  return KdTreeLinked(BuildHelper(
      0, geometry::X, KdBox{geometry::find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
