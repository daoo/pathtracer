#include "geometry/triray.h"

#include <glm/glm.hpp>

#include "geometry/ray.h"
#include "geometry/triangle.h"

using std::experimental::optional;

namespace {
constexpr float epsilon = 0.00001f;
}  // namespace

namespace geometry {
optional<TriRayIntersection> intersect(const Triangle& tri, const Ray& ray) {
  glm::vec3 e1 = tri.v1 - tri.v0;
  glm::vec3 e2 = tri.v2 - tri.v0;
  glm::vec3 q = glm::cross(ray.direction, e2);

  float a = glm::dot(e1, q);
  if (a > -epsilon && a < epsilon) return optional<TriRayIntersection>();

  glm::vec3 s = ray.origin - tri.v0;
  float f = 1.0f / a;
  float u = f * glm::dot(s, q);
  if (u < 0.0f || u > 1.0f) return optional<TriRayIntersection>();

  glm::vec3 r = glm::cross(s, e1);
  float v = f * glm::dot(ray.direction, r);
  if (v < 0.0f || u + v > 1.0f) return optional<TriRayIntersection>();

  float t = f * glm::dot(e2, r);

  return optional<TriRayIntersection>({&tri, &ray, t, u, v});
}
bool intersect_triangles(const std::vector<Triangle>& triangles,
                         const Ray& ray,
                         float mint,
                         float& maxt,
                         glm::vec3& normal,
                         const void*& tag) {
  bool hit = false;

  for (const geometry::Triangle& triangle : triangles) {
    optional<TriRayIntersection> result = intersect(triangle, ray);
    if (result && result->t >= mint && result->t <= maxt) {
      normal = result->get_normal();
      tag = triangle.tag;
      maxt = result->t;
      hit = true;
    }
  }

  return hit;
}
}  // namespace geometry
