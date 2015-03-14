#ifndef TRIANGLE_HPP_N4FJV8PZ
#define TRIANGLE_HPP_N4FJV8PZ

#include "trace/geometry/ray.hpp"
#include "trace/intersection.hpp"
#include "trace/material.hpp"
#include <glm/glm.hpp>

namespace trace
{
  struct Triangle
  {
    glm::vec3 v0, v1, v2;
    glm::vec3 n0, n1, n2;
    glm::vec2 uv0, uv1, uv2;

    const Material* material;
  };

  inline void triangle_extremes(
      const Triangle& tri,
      glm::vec3& min,
      glm::vec3& max)
  {
    min = glm::min(glm::min(tri.v0, tri.v1), tri.v2);
    max = glm::max(glm::max(tri.v0, tri.v1), tri.v2);
  }


  bool intersects(
      const Triangle& tri,
      const math::Ray& ray,
      float& t,
      glm::vec3& n);

  inline bool intersects(
      const Triangle& tri,
      const math::Ray& ray,
      float& t,
      glm::vec3& n)
  {
    constexpr float epsilon = 0.00001f;

    glm::vec3 e1 = tri.v1 - tri.v0;
    glm::vec3 e2 = tri.v2 - tri.v0;
    glm::vec3 q  = glm::cross(ray.direction, e2);

    float a = glm::dot(e1, q);
    if (a > -epsilon && a < epsilon)
      return false;

    glm::vec3 s = ray.origin - tri.v0;
    float f = 1.0f / a;
    float u = f * glm::dot(s, q);
    if (u < 0.0 || u > 1.0)
      return false;

    glm::vec3 r = glm::cross(s, e1);
    float v     = f * glm::dot(ray.direction, r);
    if (v < 0.0 || u + v > 1.0)
      return false;

    t = f * glm::dot(e2, r);
    n = glm::normalize((1.0f - (u + v)) * tri.n0 + u * tri.n1 + v * tri.n2);

    return true;
  }
}

#endif /* end of include guard: TRIANGLE_HPP_N4FJV8PZ */
