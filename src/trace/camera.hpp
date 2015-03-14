#ifndef CAMERA_HPP_6EER58HB
#define CAMERA_HPP_6EER58HB

#include <glm/glm.hpp>

namespace trace
{
  struct Camera
  {
    glm::vec3 position;
    glm::vec3 direction;
    glm::vec3 up;
    float fov;
  };

  Camera newCamera(
      const glm::vec3& position,
      const glm::vec3& target,
      const glm::vec3& approxup,
      float fov);
}

#endif /* end of include guard: CAMERA_HPP_6EER58HB */
