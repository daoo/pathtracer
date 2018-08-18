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

}  // namespace kdtree

#endif  // KDTREE_BUILD_COMMON_H_
