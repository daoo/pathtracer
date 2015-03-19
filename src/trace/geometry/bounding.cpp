#include "trace/geometry/bounding.hpp"

#include <glm/glm.hpp>

using namespace glm;
using namespace std;

namespace trace
{
  Aabb find_bounding(const vector<Triangle>& triangles)
  {
    vec3 min, max;

    for (const trace::Triangle& tri : triangles) {
      min = glm::min(min, tri.v0);
      min = glm::min(min, tri.v1);
      min = glm::min(min, tri.v2);

      max = glm::max(max, tri.v0);
      max = glm::max(max, tri.v1);
      max = glm::max(max, tri.v2);
    }

    vec3 half = (max - min) / 2.0f;
    return { min + half, half };
  }
}