#include "trace/pathtracer.h"

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
using kdtree::KdTree;
using std::optional;
using std::vector;
using trace::FastRand;
using trace::LightSample;
using trace::Material;
using trace::Scene;
using trace::SphereLight;

namespace {
constexpr float EPSILON = 0.00001f;
}  // namespace

namespace trace {

vec3 Pathtracer::LightContribution(const Scene& scene,
                                   const Material* material,
                                   const vec3& target,
                                   const vec3& offset,
                                   const vec3& wi,
                                   const vec3& n,
                                   const SphereLight& light) {
  vec3 source = light.Sample(&rand_);
  vec3 direction = source - target;
  Ray shadow_ray{offset, direction};
  if (!scene.AnyIntersect(shadow_ray, 0.0f, 1.0f)) {
    vec3 wr = normalize(direction);
    vec3 radiance = light.GetEmitted(target);
    return material->brdf(wi, wr, n) * radiance * abs(dot(wr, n));
  }
  return glm::zero<vec3>();
}

vec3 Pathtracer::EnvironmentContribution(const Ray&) const {
  return vec3(0.8f, 0.8f, 0.8f);
}

vec3 Pathtracer::Trace(const Scene& scene,
                       const Ray& ray,
                       vec3 radiance,
                       vec3 transport,
                       unsigned int bounce) {
  if (bounce >= max_bounces_) return radiance;

  optional<TriRayIntersection> intersection =
      scene.Intersect(ray, 0.0f, FLT_MAX);
  if (!intersection) return radiance + transport * EnvironmentContribution(ray);

  vec3 wi = -ray.direction;
  vec3 point = intersection->get_position();
  vec3 n = intersection->get_normal();

  const Material* material =
      static_cast<const Material*>(intersection->triangle->tag);

  vec3 offset = EPSILON * n;
  vec3 offset_up = point + offset;
  vec3 offset_down = point - offset;

  vec3 sum_lights = glm::zero<vec3>();
  for (const SphereLight& light : scene.GetLights()) {
    sum_lights +=
        LightContribution(scene, material, point, offset_up, wi, n, light);
  }

  radiance += transport * sum_lights;

  LightSample sample = material->sample_brdf(wi, n, &rand_);

  if (sample.pdf < EPSILON) return radiance;

  float cosineterm = abs(dot(sample.wo, n));
  transport = transport * (sample.brdf * (cosineterm / sample.pdf));

  if (length2(transport) < EPSILON) return radiance;

  Ray next_ray{dot(sample.wo, n) >= 0 ? offset_up : offset_down, sample.wo};

  return Trace(scene, next_ray, radiance, transport, bounce + 1);
}

vec3 Pathtracer::Trace(const Scene& scene, const Ray& ray) {
  return Trace(scene, ray, glm::zero<vec3>(), glm::one<vec3>(), 0);
}

void Pathtracer::Render(const Scene& scene,
                        const Pinhole& pinhole,
                        SampleBuffer* buffer) {
  for (unsigned int y = 0; y < buffer->height(); ++y) {
    for (unsigned int x = 0; x < buffer->width(); ++x) {
      float sx = (static_cast<float>(x) + rand_.unit()) / buffer->width();
      float sy = (static_cast<float>(y) + rand_.unit()) / buffer->height();

      buffer->add(x, y, Trace(scene, pinhole.ray(sx, sy)));
    }
  }

  buffer->inc();
}

}  // namespace trace
