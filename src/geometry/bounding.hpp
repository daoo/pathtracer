#ifndef BOUNDING_HPP_B8TQ0RYS
#define BOUNDING_HPP_B8TQ0RYS

#include "geometry/aabb.hpp"
#include <vector>

namespace geometry {
struct Triangle;

Aabb find_bounding(const std::vector<Triangle>& triangles);
}

#endif /* end of include guard: BOUNDING_HPP_B8TQ0RYS */
