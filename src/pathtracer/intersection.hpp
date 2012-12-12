#ifndef INTERSECTION_HPP_B7YTSMBV
#define INTERSECTION_HPP_B7YTSMBV

#include "pathtracer/material.hpp"

#include <glm/glm.hpp>

struct Intersection {
  glm::vec3 m_position;
  glm::vec3 m_normal;
  const Material* m_material;
};

#endif /* end of include guard: INTERSECTION_HPP_B7YTSMBV */
