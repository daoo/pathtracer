#ifndef LIGHT_HPP_JKOQK7HA
#define LIGHT_HPP_JKOQK7HA

#include "math/helpers.hpp"
#include "trace/fastrand.hpp"
#include "trace/mcsampling.hpp"
#include <glm/glm.hpp>

namespace trace
{
  /**
   * For now, a light is an emitting sphere that radiates uniformly in all
   * directions.
   */
  struct SphereLight
  {
    float radius;
    glm::vec3 position;
    glm::vec3 intensity;
  };

  glm::vec3 lightSample(const SphereLight&);
  glm::vec3 lightEmitted(const SphereLight&, const glm::vec3&);

  inline glm::vec3 lightSample(FastRand& rand, const SphereLight& light)
  {
    return light.position + uniformSampleSphere(rand, light.radius);
  }

  /**
   * Calculate the radiance that is emitted from the light and reaches point p.
   */
  inline glm::vec3 lightEmitted(const SphereLight& light, const glm::vec3& p)
  {
    return light.intensity / math::lengthSquared(light.position - p);
  }
}

#endif /* end of include guard: LIGHT_HPP_JKOQK7HA */
