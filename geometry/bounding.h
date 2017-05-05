#ifndef GEOMETRY_BOUNDING_H_
#define GEOMETRY_BOUNDING_H_

#include <vector>

namespace geometry {
class Aabb;
struct Triangle;

Aabb find_bounding(const std::vector<Triangle>& triangles);
}  // namespace geometry

#endif  // GEOMETRY_BOUNDING_H_
