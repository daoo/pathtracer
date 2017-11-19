#include "kdtree/build.h"

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include <cassert>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "geometry/split.h"
#include "geometry/tribox.h"
#include "kdtree/axis.h"
#include "kdtree/intersect.h"
#include "kdtree/linked.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

using geometry::Aabb;
using geometry::AabbSplit;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using glm::vec3;
using kdtree::KdNodeLinked;
using std::vector;

namespace {

struct Box {
  geometry::Aabb boundary;
  std::vector<const geometry::Triangle*> triangles;
};

struct Split {
  geometry::Aap plane;
  Box left, right;
};

Split split_box(const Box& parent, const Aap& plane) {
  AabbSplit aabbs = split(parent.boundary, plane, glm::epsilon<float>());
  kdtree::IntersectResults triangles =
      kdtree::intersect_test(parent.triangles, aabbs.left, aabbs.right);
  Box left{aabbs.left, triangles.left};
  Box right{aabbs.right, triangles.right};
  return Split{plane, left, right};
}

KdNodeLinked* go(unsigned int depth, Axis axis, const Box& parent) {
  if (depth >= 20 || parent.triangles.size() <= 6) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  } else {
    Aap plane(axis, parent.boundary.GetCenter()[axis]);
    Split split = split_box(parent, plane);
    KdNodeLinked* left_child =
        go(depth + 1, kdtree::next_axis(axis), split.left);
    KdNodeLinked* right_child =
        go(depth + 1, kdtree::next_axis(axis), split.right);
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

  return KdTreeLinked(go(0, geometry::X, Box{find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
