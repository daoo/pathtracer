#ifndef INTERSECTION_HPP_B7YTSMBV
#define INTERSECTION_HPP_B7YTSMBV

#include <glm/glm.hpp>

class Material;

struct Intersection {
  glm::vec3 m_position;
  glm::vec3 m_normal;
  Material* m_material;
};

#endif /* end of include guard: INTERSECTION_HPP_B7YTSMBV */
