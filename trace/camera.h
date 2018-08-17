#ifndef TRACE_CAMERA_H_
#define TRACE_CAMERA_H_

#include <glm/glm.hpp>

#include "geometry/ray.h"

namespace trace {
struct Camera {
  glm::vec3 position;
  glm::vec3 direction;
  glm::vec3 up;
  glm::vec3 right;
  float fov;

  Camera(const glm::vec3& position_,
         const glm::vec3& target,
         const glm::vec3& up_,
         float fov_)
      : position(position_),
        direction(glm::normalize(target - position_)),
        up(glm::normalize(up_)),
        right(glm::normalize(glm::cross(direction, up))),
        fov(fov_) {}
};

struct Pinhole {
  glm::vec3 position;
  glm::vec3 mind;
  glm::vec3 dx, dy;

  Pinhole(const Camera& camera, float aspect_ratio)
      : position(camera.position) {
    float fov_half = camera.fov / 2.0f;

    glm::vec3 x = camera.up * glm::sin(fov_half);
    glm::vec3 y = camera.right * glm::sin(fov_half) * aspect_ratio;
    glm::vec3 z = camera.direction * glm::cos(fov_half);

    mind = z - y - x;

    dx = 2.0f * y;
    dy = 2.0f * x;
  }

  geometry::Ray ray(float x, float y) const {
    return {position, glm::normalize(mind + x * dx + y * dy)};
  }
};
}  // namespace trace

#endif  // TRACE_CAMERA_H_
