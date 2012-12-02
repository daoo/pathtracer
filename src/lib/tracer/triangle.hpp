#ifndef TRIANGLE_HPP_N4FJV8PZ
#define TRIANGLE_HPP_N4FJV8PZ

#include "math/ray.hpp"
#include "tracer/material.hpp"
#include <glm/glm.hpp>

class Triangle {
  public:
    glm::vec3 v0, v1, v2;
    glm::vec3 n0, n1, n2;
    glm::vec2 uv0, uv1, uv2;

    const Material* m_material;
};

bool intersects(const Triangle& tri, math::Ray& r, Intersection& i);

inline bool intersects(const Triangle& tri, math::Ray& r, Intersection& i) {
  constexpr float epsilon = 0.00001f;

  glm::vec3 d  = r.d;
  glm::vec3 o  = r.o;
  glm::vec3 e1 = tri.v1 - tri.v0;
  glm::vec3 e2 = tri.v2 - tri.v0;
  glm::vec3 q  = glm::cross(d, e2);

  float a = glm::dot(e1, q);
  if (a > -epsilon && a < epsilon)
    return false;

  glm::vec3 s = o - tri.v0;
  float f = 1.0f / a;
  float u = f * glm::dot(s, q);
  if (u < 0.0 || u > 1.0)
    return false;

  glm::vec3 R = glm::cross(s, e1);
  float v     = f * glm::dot(d, R);
  if (v < 0.0 || u + v > 1.0)
    return false;

  float t = f * glm::dot(e2, R);
  if (t < r.mint || t > r.maxt)
    return false;

  r.maxt       = t;
  i.m_position = r(t);
  i.m_normal   = glm::normalize((1.0f - (u + v)) * tri.n0 + u * tri.n1 + v * tri.n2);
  i.m_material = tri.m_material;
  return true;
}


#endif /* end of include guard: TRIANGLE_HPP_N4FJV8PZ */
