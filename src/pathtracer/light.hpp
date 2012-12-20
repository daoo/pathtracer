#ifndef LIGHT_HPP_JKOQK7HA
#define LIGHT_HPP_JKOQK7HA

#include "math/helpers.hpp"

#include <glm/glm.hpp>
#include <glm/gtx/random.hpp>

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

inline glm::vec3 lightSample(const SphereLight& light)
{
  // TODO: Use thread local random engine
  return light.position + glm::sphericalRand(light.radius);
}

/**
 * Calculate the radiance that is emitted from the light and reaches point p.
 */
inline glm::vec3 lightEmitted(const SphereLight& light, const glm::vec3& p)
{
  return light.intensity / math::lengthSquared(light.position - p);
}

#endif /* end of include guard: LIGHT_HPP_JKOQK7HA */
