#include "geometry/bounding.h"

#include <glm/glm.hpp>

#include "geometry/aabb.h"
#include "geometry/triangle.h"

using glm::vec3;
using std::vector;

namespace geometry {
Aabb find_bounding(const vector<Triangle>& triangles) {
  vec3 min, max;

  for (const Triangle& tri : triangles) {
    min = glm::min(min, tri.v0);
    min = glm::min(min, tri.v1);
    min = glm::min(min, tri.v2);

    max = glm::max(max, tri.v0);
    max = glm::max(max, tri.v1);
    max = glm::max(max, tri.v2);
  }

  vec3 half = (max - min) / 2.0f;
  return {min + half, half};
}
}  // namespace geometry
