#ifndef KDTREE_NAIVE_H_
#define KDTREE_NAIVE_H_

#include <vector>

namespace geometry {
struct Triangle;
}  // namespace geometry

namespace kdtree {
class KdNodeLinked;
KdNodeLinked* build_tree_naive(
    const std::vector<geometry::Triangle>& triangles);
}  // namespace kdtree

#endif  // KDTREE_NAIVE_H_
