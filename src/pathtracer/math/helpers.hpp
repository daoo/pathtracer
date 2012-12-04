#ifndef HELPERS_HPP_5NWOANYM
#define HELPERS_HPP_5NWOANYM

#include <glm/glm.hpp>

namespace math {
  inline float lengthSquared(const glm::vec3& v) {
    return v.x * v.x + v.y * v.y + v.z * v.z;
  }
}

#endif /* end of include guard: HELPERS_HPP_5NWOANYM */
