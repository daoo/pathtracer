#ifndef KDTREE_SURFACE_AREA_HEURISTIC_HPP
#define KDTREE_SURFACE_AREA_HEURISTIC_HPP

#include "geometry/triangle.hpp"
#include "kdtree/linked.hpp"
#include <vector>

namespace kdtree {
KdTreeLinked build_tree_sah(const std::vector<geometry::Triangle>& triangles);
}

#endif // KDTREE_SURFACE_AREA_HEURISTIC_HPP
