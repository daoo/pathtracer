#ifndef GEOMETRY_RAY_H_
#define GEOMETRY_RAY_H_

#include <glm/glm.hpp>

namespace geometry {
struct Ray {
  glm::vec3 origin, direction;

  glm::vec3 param(float t) const { return origin + t * direction; }
};
}  // namespace geometry

#endif  // GEOMETRY_RAY_H_
