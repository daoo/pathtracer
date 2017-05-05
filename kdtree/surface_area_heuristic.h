#ifndef KDTREE_SURFACE_AREA_HEURISTIC_H_
#define KDTREE_SURFACE_AREA_HEURISTIC_H_

#include <vector>

namespace geometry {
struct Triangle;
}  // namespace geometry

namespace kdtree {
class KdTreeLinked;
KdTreeLinked build_tree_sah(const std::vector<geometry::Triangle>& triangles);
}  // namespace kdtree

#endif  // KDTREE_SURFACE_AREA_HEURISTIC_H_
