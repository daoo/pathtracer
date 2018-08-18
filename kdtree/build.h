#ifndef KDTREE_BUILD_H_
#define KDTREE_BUILD_H_

#include <vector>

namespace geometry {
struct Triangle;
}  // namespace geometry

namespace kdtree {
class KdTree;
KdTree build(const std::vector<geometry::Triangle>& triangles);
}  // namespace kdtree

#endif  // KDTREE_BUILD_H_
