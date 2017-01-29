#ifndef GEOMETRY_TRIRAY_H_
#define GEOMETRY_TRIRAY_H_

#include <glm/glm.hpp>

#include "geometry/ray.h"
#include "geometry/triangle.h"

namespace geometry {
inline bool triray(const Triangle& tri,
                   const Ray& ray,
                   float& t,
                   glm::vec3& n) {
  constexpr float epsilon = 0.00001f;

  glm::vec3 e1 = tri.v1 - tri.v0;
  glm::vec3 e2 = tri.v2 - tri.v0;
  glm::vec3 q = glm::cross(ray.direction, e2);

  float a = glm::dot(e1, q);
  if (a > -epsilon && a < epsilon) return false;

  glm::vec3 s = ray.origin - tri.v0;
  float f = 1.0f / a;
  float u = f * glm::dot(s, q);
  if (u < 0.0f || u > 1.0f) return false;

  glm::vec3 r = glm::cross(s, e1);
  float v = f * glm::dot(ray.direction, r);
  if (v < 0.0f || u + v > 1.0f) return false;

  t = f * glm::dot(e2, r);
  n = glm::normalize((1.0f - (u + v)) * tri.n0 + u * tri.n1 + v * tri.n2);

  return true;
}
}  // namespace geometry

#endif  // GEOMETRY_TRIRAY_H_
