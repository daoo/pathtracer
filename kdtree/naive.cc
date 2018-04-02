#include "kdtree/build.h"

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "geometry/split.h"
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

struct KdBox {
  Aabb boundary;
  std::vector<const Triangle*> triangles;
};

struct KdSplit {
  Aap plane;
  KdBox left, right;
};

KdSplit Split(const KdBox& parent, const Aap& plane) {
  AabbSplit aabbs = geometry::Split(parent.boundary, plane);
  kdtree::IntersectResults triangles =
      kdtree::PartitionTriangles(parent.triangles, plane);
  KdBox left{aabbs.left, triangles.left};
  KdBox right{aabbs.right, triangles.right};
  return KdSplit{plane, left, right};
}

KdNodeLinked* BuildHelper(unsigned int depth, Axis axis, const KdBox& parent) {
  if (depth >= 20 || parent.triangles.size() <= 6) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  } else {
    Aap plane(axis, parent.boundary.GetCenter()[axis]);
    KdSplit split = Split(parent, plane);
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
