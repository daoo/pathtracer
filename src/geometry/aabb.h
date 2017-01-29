#ifndef GEOMETRY_AABB_H_
#define GEOMETRY_AABB_H_

#include <glm/glm.hpp>

namespace geometry {
struct Aabb {
  glm::vec3 center;
  glm::vec3 half;

  inline float surface_area() const {
    return 8.0f * (half.x * half.y + half.x * half.z + half.y * half.z);
  }
};
}  // namespace geometry

#endif  // GEOMETRY_AABB_H_
