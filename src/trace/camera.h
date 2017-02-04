#ifndef TRACE_CAMERA_H_
#define TRACE_CAMERA_H_

#include <glm/glm.hpp>

#include "geometry/ray.h"

namespace trace {
struct Camera {
  glm::vec3 position;
  glm::vec3 direction;
  glm::vec3 right;
  glm::vec3 up;
  float fov;

  Camera(const glm::vec3& position_,
         const glm::vec3& target,
         const glm::vec3& approxup,
         float fov_)
      : position(position_),
        direction(glm::normalize(target - position_)),
        right(glm::normalize(glm::cross(direction, glm::normalize(approxup)))),
        up(glm::normalize(glm::cross(right, direction))),
        fov(fov_) {}
};

struct Pinhole {
  float width, height;

  glm::vec3 position;
  glm::vec3 mind;
  glm::vec3 dx, dy;

  Pinhole(const Camera& camera, unsigned int int_width, unsigned int int_height)
      : width(static_cast<float>(int_width)),
        height(static_cast<float>(int_height)),
        position(camera.position) {
    float aspect = width / height;
    float fov_half = camera.fov / 2.0f;

    glm::vec3 x = camera.up * glm::sin(fov_half);
    glm::vec3 y = camera.right * glm::sin(fov_half) * aspect;
    glm::vec3 z = camera.direction * glm::cos(fov_half);

    mind = z - y - x;

    dx = 2.0f * ((z - x) - mind);
    dy = 2.0f * ((z - y) - mind);
  }

  geometry::Ray ray(float x, float y) const {
    return {position, normalize(mind + x * dx + y * dy)};
  }
};
}  // namespace trace

#endif  // TRACE_CAMERA_H_
