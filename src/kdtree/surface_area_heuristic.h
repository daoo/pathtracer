#ifndef KDTREE_SURFACE_AREA_HEURISTIC_H_
#define KDTREE_SURFACE_AREA_HEURISTIC_H_

#include <vector>

#include "kdtree/linked.h"

namespace geometry {
struct Triangle;
}

namespace kdtree {
KdTreeLinked build_tree_sah(const std::vector<geometry::Triangle>& triangles);
}

#endif  // KDTREE_SURFACE_AREA_HEURISTIC_H_
