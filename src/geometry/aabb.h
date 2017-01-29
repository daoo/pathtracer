#ifndef AABB_HPP_VOTKJJ0Y
#define AABB_HPP_VOTKJJ0Y

#include <glm/glm.hpp>

namespace geometry {
struct Aabb {
  glm::vec3 center;
  glm::vec3 half;

  inline float surface_area() const {
    return 8.0f * (half.x * half.y + half.x * half.z + half.y * half.z);
  }
};
}

#endif /* end of include guard: AABB_HPP_VOTKJJ0Y */
