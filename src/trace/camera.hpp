#ifndef CAMERA_HPP_6EER58HB
#define CAMERA_HPP_6EER58HB

#include "geometry/ray.hpp"
#include <glm/glm.hpp>

namespace trace {
struct Camera {
  glm::vec3 position;
  glm::vec3 direction;
  glm::vec3 approxup;
  float fov;

  Camera(const glm::vec3& position,
         const glm::vec3& target,
         const glm::vec3& approxup,
         float fov)
      : position(position),
        direction(glm::normalize(target - position)),
        approxup(glm::normalize(approxup)),
        fov(fov) {}
};

struct Pinhole {
  float width, height;

  glm::vec3 position;
  glm::vec3 mind;
  glm::vec3 dx, dy;

  Pinhole(const Camera& camera, unsigned int int_width, unsigned int int_height)
      : width(static_cast<float>(int_width)),
        height(static_cast<float>(int_height)) {
    glm::vec3 right =
        glm::normalize(glm::cross(camera.direction, camera.approxup));
    glm::vec3 up = glm::normalize(glm::cross(right, camera.direction));

    float aspect = width / height;
    float fov_half = camera.fov / 2.0f;

    glm::vec3 x = up * glm::sin(fov_half);
    glm::vec3 y = right * glm::sin(fov_half) * aspect;
    glm::vec3 z = camera.direction * glm::cos(fov_half);

    mind = z - y - x;

    dx = 2.0f * ((z - x) - mind);
    dy = 2.0f * ((z - y) - mind);
  }

  Ray ray(float x, float y) const {
    return {position, normalize(mind + x * dx + y * dy)};
  }
};
}

#endif /* end of include guard: CAMERA_HPP_6EER58HB */
