#ifndef GEOMETRY_TRIANGLE_H_
#define GEOMETRY_TRIANGLE_H_

#include <glm/glm.hpp>

namespace geometry {
struct Triangle {
  glm::vec3 v0, v1, v2;
  glm::vec3 n0, n1, n2;
  glm::vec2 uv0, uv1, uv2;

  const void* tag;
};

inline void triangle_extremes(const Triangle& tri,
                              glm::vec3& min,
                              glm::vec3& max) {
  min = glm::min(glm::min(tri.v0, tri.v1), tri.v2);
  max = glm::max(glm::max(tri.v0, tri.v1), tri.v2);
}
}  // namespace geometry

#endif  // GEOMETRY_TRIANGLE_H_
