#ifndef AABB_HPP_VOTKJJ0Y
#define AABB_HPP_VOTKJJ0Y

#include <glm/glm.hpp>

namespace math {
  struct Aabb {
    glm::vec3 min;
    glm::vec3 max;
  };

  glm::vec3 center(const Aabb&);
  glm::vec3 half_size(const Aabb&);

  float volume(const Aabb&);
  float area(const Aabb&);

  Aabb combine(const Aabb&, const Aabb&);
  Aabb combine(const Aabb&, const glm::vec3&);

  Aabb make_aabb(const glm::vec3&, const float);
  Aabb make_aabb(const glm::vec3&, const glm::vec3&);
  Aabb make_aabb(const glm::vec3*, const size_t);

  Aabb make_inverse_extreme_aabb();
}

#endif /* end of include guard: AABB_HPP_VOTKJJ0Y */
