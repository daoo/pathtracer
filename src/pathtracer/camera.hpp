#ifndef CAMERA_HPP_6EER58HB
#define CAMERA_HPP_6EER58HB

#include <glm/glm.hpp>

struct Camera
{
  glm::vec3 position;
  glm::vec3 direction;
  glm::vec3 up;
  float fov;
};

#endif /* end of include guard: CAMERA_HPP_6EER58HB */
