#ifndef TRIANGLE_HPP_N4FJV8PZ
#define TRIANGLE_HPP_N4FJV8PZ

#include "trace/material.hpp"
#include <glm/glm.hpp>

namespace trace {
struct Triangle {
  glm::vec3 v0, v1, v2;
  glm::vec3 n0, n1, n2;
  glm::vec2 uv0, uv1, uv2;

  const Material* material;
};

inline void triangle_extremes(const Triangle& tri,
                              glm::vec3& min,
                              glm::vec3& max) {
  min = glm::min(glm::min(tri.v0, tri.v1), tri.v2);
  max = glm::max(glm::max(tri.v0, tri.v1), tri.v2);
}
}

#endif /* end of include guard: TRIANGLE_HPP_N4FJV8PZ */
