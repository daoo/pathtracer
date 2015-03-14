#ifndef LIGHT_HPP_JKOQK7HA
#define LIGHT_HPP_JKOQK7HA

#include "trace/fastrand.hpp"
#include "trace/mcsampling.hpp"
#include <glm/glm.hpp>
#include <glm/gtx/norm.hpp>

namespace trace
{
  /**
   * An emitting sphere that radiates uniformly in all directions.
   */
  struct SphereLight
  {
    float radius;
    glm::vec3 center;
    glm::vec3 intensity;
  };

  SphereLight newLight(
      const glm::vec3& center,
      const glm::vec3& color,
      float intensity,
      float radius);

  glm::vec3 lightSample(const SphereLight&);
  glm::vec3 lightEmitted(const SphereLight&, const glm::vec3&);

  inline glm::vec3 lightSample(FastRand& rand, const SphereLight& light)
  {
    return light.center + uniformSampleSphere(rand) * light.radius;
  }

  /**
   * Calculate the radiance that is emitted from the light and reaches point p.
   */
  inline glm::vec3 lightEmitted(const SphereLight& light, const glm::vec3& p)
  {
    return light.intensity / glm::length2(light.center - p);
  }
}

#endif /* end of include guard: LIGHT_HPP_JKOQK7HA */
