#ifndef AABB_HPP_VOTKJJ0Y
#define AABB_HPP_VOTKJJ0Y

#include <glm/glm.hpp>

namespace geometry {
struct Aabb {
  glm::vec3 center;
  glm::vec3 half;
};

inline float surface_area(const Aabb& box) {
  return 8.0f * (box.half.x * box.half.y + box.half.x * box.half.z +
                 box.half.y * box.half.z);
}
}

#endif /* end of include guard: AABB_HPP_VOTKJJ0Y */
