#ifndef KDTREE_SURFACE_AREA_HEURISTIC_H_
#define KDTREE_SURFACE_AREA_HEURISTIC_H_

#include <vector>

namespace geometry {
struct Triangle;
}

namespace kdtree {
struct KdNodeLinked;
KdNodeLinked* build_tree_sah(const std::vector<geometry::Triangle>& triangles);
}

#endif  // KDTREE_SURFACE_AREA_HEURISTIC_H_
