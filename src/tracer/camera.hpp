#ifndef CAMERA_HPP_6EER58HB
#define CAMERA_HPP_6EER58HB

#include <glm/glm.hpp>

struct Camera {
  glm::vec3 m_position;
  glm::vec3 m_direction;
  glm::vec3 m_up;
  float m_fov;
};

#endif /* end of include guard: CAMERA_HPP_6EER58HB */
