#ifndef RAY_HPP_CNODB0L7
#define RAY_HPP_CNODB0L7

#include <glm/glm.hpp>

namespace trace {
struct Ray {
  glm::vec3 origin, direction;

  glm::vec3 param(float t) const { return origin + t * direction; }
};
}

#endif /* end of include guard: RAY_HPP_CNODB0L7 */
