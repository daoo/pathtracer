#ifndef KDTREE_BUILD_COMMON_H_
#define KDTREE_BUILD_COMMON_H_

#include "geometry/aabb.h"
#include "geometry/split.h"
#include "kdtree/intersect.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

namespace kdtree {

struct KdBox {
  geometry::Aabb boundary;
  std::vector<const geometry::Triangle*> triangles;
};

struct KdSplit {
  geometry::Aap plane;
  KdBox left, right;
};

KdSplit Split(const KdBox& parent, const geometry::Aap& plane) {
  geometry::AabbSplit aabbs = geometry::Split(parent.boundary, plane);
  kdtree::IntersectResults triangles =
      kdtree::PartitionTriangles(parent.triangles, plane);
  KdBox left{aabbs.left, triangles.left};
  KdBox right{aabbs.right, triangles.right};
  return KdSplit{plane, left, right};
}

}  // namespace kdtree

#endif  // KDTREE_BUILD_COMMON_H_
