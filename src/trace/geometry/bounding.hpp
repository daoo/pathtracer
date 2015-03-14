#ifndef BOUNDING_HPP_B8TQ0RYS
#define BOUNDING_HPP_B8TQ0RYS

#include "trace/geometry/aabb.hpp"
#include "trace/geometry/triangle.hpp"
#include <vector>

namespace math
{
  Aabb find_bounding(const std::vector<trace::Triangle>& triangles);
}

#endif /* end of include guard: BOUNDING_HPP_B8TQ0RYS */
