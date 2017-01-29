#ifndef GEOMETRY_BOUNDING_H_
#define GEOMETRY_BOUNDING_H_

#include <vector>

#include "geometry/aabb.h"

namespace geometry {
struct Triangle;

Aabb find_bounding(const std::vector<Triangle>& triangles);
}  // namespace geometry

#endif  // GEOMETRY_BOUNDING_H_
