#ifndef KDTREE_UTIL_H_
#define KDTREE_UTIL_H_

#include <vector>

#include "geometry/aabb.h"
#include "geometry/aap.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

namespace kdtree {
struct Box {
  geometry::Aabb boundary;
  std::vector<const geometry::Triangle*> triangles;
};

struct Split {
  geometry::Aap plane;
  Box left, right;
};

Split split_box(const Box& parent, const geometry::Aap& plane);
}  // namespace kdtree

#endif  // KDTREE_UTIL_H_
