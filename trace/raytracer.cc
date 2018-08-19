#include "trace/raytracer.h"

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include <optional>

#include "geometry/ray.h"
#include "geometry/triray.h"
#include "kdtree/kdtree.h"
#include "trace/camera.h"
#include "trace/fastrand.h"
#include "trace/light.h"
#include "trace/material.h"
#include "trace/mcsampling.h"
#include "trace/samplebuffer.h"
#include "trace/scene.h"

using geometry::Ray;
using geometry::TriRayIntersection;
using glm::vec3;
using std::optional;
using trace::FastRand;
using trace::Material;
using trace::Scene;
using trace::SphereLight;

namespace {
constexpr float EPSILON = 0.00001f;
}  // namespace

namespace trace {

vec3 Raytracer::LightContribution(const Scene& scene,
                                  const Material* material,
                                  const vec3& target,
                                  const vec3& offset,
                                  const vec3& wi,
                                  const vec3& n,
                                  const SphereLight& light) {
  vec3 source = light.GetCenter();
  vec3 direction = source - target;
  Ray shadow_ray{offset, direction};
  if (!scene.AnyIntersect(shadow_ray, 0.0f, 1.0f)) {
    vec3 wr = glm::normalize(direction);
    vec3 radiance = light.GetEmitted(target);
    return material->GetBrdf(wi, wr, n) * radiance * abs(dot(wr, n));
  }
  return glm::zero<vec3>();
}

vec3 Raytracer::EnvironmentContribution(const Ray&) const {
  return vec3(0.8f, 0.8f, 0.8f);
}

vec3 Raytracer::Trace(const Scene& scene, const Ray& ray) {
  optional<TriRayIntersection> intersection =
      scene.Intersect(ray, 0.0f, FLT_MAX);
  if (!intersection) return EnvironmentContribution(ray);

  vec3 wi = -ray.direction;
  vec3 point = intersection->get_position();
  vec3 n = intersection->get_normal();

  vec3 offset = point + EPSILON * n;

  const Material* material =
      static_cast<const Material*>(intersection->triangle->tag);

  vec3 radiance = glm::zero<vec3>();
  for (const SphereLight& light : scene.GetLights()) {
    radiance += LightContribution(scene, material, point, offset, wi, n, light);
  }

  return radiance;
}

void Raytracer::Render(const Scene& scene,
                       const Pinhole& pinhole,
                       SampleBuffer* buffer) {
  for (unsigned int y = 0; y < buffer->height(); ++y) {
    for (unsigned int x = 0; x < buffer->width(); ++x) {
      float sx = (static_cast<float>(x) + 0.5f) / buffer->width();
      float sy = (static_cast<float>(y) + 0.5f) / buffer->height();
      buffer->add(x, y, Trace(scene, pinhole.ray(sx, sy)));
    }
  }

  buffer->inc();
}

}  // namespace trace
