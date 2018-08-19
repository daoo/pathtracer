#include "geometry/bounding.h"

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include <algorithm>

#include "geometry/aabb.h"
#include "geometry/triangle.h"

using glm::vec3;
using std::vector;

namespace geometry {
Aabb find_bounding(const vector<Triangle>& triangles) {
  if (triangles.size() == 0) {
    return {glm::zero<vec3>(), glm::zero<vec3>()};
  }

  vec3 a = triangles[0].GetMin();
  vec3 b = triangles[0].GetMax();

  for (size_t i = 1; i < triangles.size(); ++i) {
    const Triangle& tri = triangles[i];
    a = glm::min(a, tri.GetMin());
    b = glm::max(b, tri.GetMax());
  }

  vec3 half = (b - a) / 2.0f;
  return {a + half, half};
}
}  // namespace geometry
