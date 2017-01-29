#ifndef KDTREE_NAIVE_H_
#define KDTREE_NAIVE_H_

#include <vector>

#include "kdtree/linked.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

namespace kdtree {
KdTreeLinked build_tree_naive(const std::vector<geometry::Triangle>& triangles);
}  // namespace kdtree

#endif  // KDTREE_NAIVE_H_
