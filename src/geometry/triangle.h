#ifndef GEOMETRY_TRIANGLE_H_
#define GEOMETRY_TRIANGLE_H_

#include <glm/glm.hpp>

namespace geometry {
struct Triangle {
  glm::vec3 v0, v1, v2;
  glm::vec3 n0, n1, n2;
  glm::vec2 uv0, uv1, uv2;

  const void* tag;

  inline glm::vec3 GetMin() const { return glm::min(glm::min(v0, v1), v2); }
  inline glm::vec3 GetMax() const { return glm::max(glm::max(v0, v1), v2); }
};
}  // namespace geometry

#endif  // GEOMETRY_TRIANGLE_H_
