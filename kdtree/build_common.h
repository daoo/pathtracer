#ifndef KDTREE_BUILD_COMMON_H_
#define KDTREE_BUILD_COMMON_H_

#include <vector>

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

enum Side { LEFT, RIGHT };

KdSplit Split(const KdBox& parent, const geometry::Aap& plane, Side side) {
  geometry::AabbSplit aabbs = geometry::Split(parent.boundary, plane);
  kdtree::IntersectResults triangles =
      kdtree::PartitionTriangles(parent.triangles, plane);
  std::vector<const geometry::Triangle*> left_tris(triangles.left);
  std::vector<const geometry::Triangle*> right_tris(triangles.right);
  if (side == LEFT) {
    left_tris.insert(left_tris.end(), triangles.plane.cbegin(),
                     triangles.plane.cend());
  } else if (side == RIGHT) {
    right_tris.insert(right_tris.end(), triangles.plane.cbegin(),
                      triangles.plane.cend());
  } else {
    assert(false);
  }
  KdBox left{aabbs.left, left_tris};
  KdBox right{aabbs.right, right_tris};
  return KdSplit{plane, left, right};
}

}  // namespace kdtree

#endif  // KDTREE_BUILD_COMMON_H_
