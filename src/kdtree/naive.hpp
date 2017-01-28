#ifndef BUILD_HPP_EBQD1OY8
#define BUILD_HPP_EBQD1OY8

#include "kdtree/linked.hpp"
#include <vector>

namespace geometry {
struct Triangle;
}

namespace kdtree {
KdTreeLinked build_tree_naive(const std::vector<geometry::Triangle>& triangles);
}

#endif /* end of include guard: BUILD_HPP_EBQD1OY8 */
