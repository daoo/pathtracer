#include "geometry/triray.h"

#include <glm/glm.hpp>

#include "geometry/ray.h"
#include "geometry/triangle.h"

namespace {
constexpr float epsilon = 0.00001f;
}  // namespace

namespace geometry {
bool triray(const Triangle& tri, const Ray& ray, float& t, glm::vec3& n) {
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

bool intersect_triangles(const std::vector<Triangle>& triangles,
                         const Ray& ray,
                         float mint,
                         float& maxt,
                         glm::vec3& normal,
                         const void*& tag) {
  bool hit = false;

  for (const geometry::Triangle& triangle : triangles) {
    float t;
    glm::vec3 n;
    if (triray(triangle, ray, t, n)) {
      if (t >= mint && t <= maxt) {
        normal = n;
        tag = triangle.tag;

        maxt = t;

        hit = true;
      }
    }
  }

  return hit;
}
}  // namespace geometry
