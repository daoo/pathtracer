#ifndef LIGHT_HPP_JKOQK7HA
#define LIGHT_HPP_JKOQK7HA

#include "math/helpers.hpp"
#include "mcsampling.hpp"

#include <glm/glm.hpp>
#include <glm/gtx/constants.hpp>
#include <glm/gtx/random.hpp>

/**
 * For now, a light is an emitting sphere that radiates uniformly in all
 * directions.
 */
struct Light {
  float m_radius;
  glm::vec3 m_position;
  glm::vec3 m_intensity;
};

glm::vec3 sample(const Light&);
glm::vec3 Le(const Light&, const glm::vec3&);

/**
 * Rejection sample a uniform position inside the sphere. There is
 * a better way.
 */
inline glm::vec3 sample(const Light& light) {
  return light.m_position + glm::sphericalRand(light.m_radius);
}

/**
 * Calculate the radiance that is emitted from the light and reaches point p.
 */
inline glm::vec3 Le(const Light& light, const glm::vec3& p) {
  return light.m_intensity / math::lengthSquared(light.m_position - p);
}

#endif /* end of include guard: LIGHT_HPP_JKOQK7HA */
