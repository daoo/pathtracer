#ifndef TRACE_LIGHT_H_
#define TRACE_LIGHT_H_

#define GLM_ENABLE_EXPERIMENTAL

#include <glm/glm.hpp>
#include <glm/gtx/norm.hpp>

#include "trace/fastrand.h"
#include "trace/mcsampling.h"

namespace trace {
/**
 * An emitting sphere that radiates uniformly in all directions.
 */
class SphereLight {
 public:
  SphereLight(const glm::vec3& center,
              const glm::vec3& color,
              float intensity,
              float radius)
      : radius_(radius), center_(center), intensity_(intensity * color) {}

  inline glm::vec3 Sample(FastRand* rand) const {
    return center_ + uniform_sample_sphere(rand) * radius_;
  }

  /**
   * Calculate the radiance that is emitted from the light to a point.
   */
  inline glm::vec3 GetEmitted(const glm::vec3& point) const {
    return intensity_ / glm::length2(center_ - point);
  }

 private:
  float radius_;
  glm::vec3 center_;
  glm::vec3 intensity_;
};
}  // namespace trace

#endif  // TRACE_LIGHT_H_
