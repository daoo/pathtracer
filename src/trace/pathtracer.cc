#include "trace/pathtracer.h"

#include <experimental/optional>
#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include "geometry/ray.h"
#include "geometry/triray.h"
#include "kdtree/intersection.h"
#include "kdtree/linked.h"
#include "trace/camera.h"
#include "trace/fastrand.h"
#include "trace/light.h"
#include "trace/material.h"
#include "trace/mcsampling.h"
#include "trace/samplebuffer.h"

using geometry::Ray;
using geometry::TriRayIntersection;
using glm::vec3;
using kdtree::KdTreeLinked;
using std::experimental::optional;
using std::vector;
using trace::FastRand;
using trace::LightSample;
using trace::Material;
using trace::SphereLight;

namespace {
constexpr float EPSILON = 0.00001f;

bool any_intersects(const KdTreeLinked& kdtree,
                    const Ray& ray,
                    float tmin,
                    float tmax) {
  return static_cast<bool>(search_tree(kdtree, ray, tmin, tmax));
}

vec3 from_light(const KdTreeLinked& kdtree,
                const Material* material,
                const vec3& target,
                const vec3& offset,
                const vec3& wi,
                const vec3& n,
                const SphereLight& light,
                FastRand* rand) {
  vec3 source = light.light_sample(rand);
  vec3 direction = source - target;
  Ray shadow_ray{offset, direction};
  if (!any_intersects(kdtree, shadow_ray, 0.0f, 1.0f)) {
    vec3 wr = normalize(direction);
    vec3 radiance = light.light_emitted(target);
    return material->brdf(wi, wr, n) * radiance * abs(dot(wr, n));
  }
  return glm::zero<vec3>();
}

vec3 environment_light(const Ray&) {
  return vec3(0.8f, 0.8f, 0.8f);
}

struct Context {
  const KdTreeLinked& kdtree;
  const vector<SphereLight>& lights;
  const unsigned int bounces;
  FastRand* rand;
};

vec3 incoming_light_helper(const Context& context,
                           const Ray& ray,
                           vec3 radiance,
                           vec3 transport,
                           unsigned int bounce) {
  if (bounce >= context.bounces) return radiance;

  optional<TriRayIntersection> intersection =
      kdtree::search_tree(context.kdtree, ray, 0.0f, FLT_MAX);
  if (!intersection) return radiance + transport * environment_light(ray);

  vec3 wi = -ray.direction;
  vec3 point = intersection->get_position();
  vec3 n = intersection->get_normal();

  const Material* material =
      static_cast<const Material*>(intersection->triangle->tag);

  vec3 offset = EPSILON * n;
  vec3 offset_up = point + offset;
  vec3 offset_down = point - offset;

  vec3 sum_lights = glm::zero<vec3>();
  for (const SphereLight& light : context.lights) {
    sum_lights += from_light(context.kdtree, material, point, offset_up, wi, n,
                             light, context.rand);
  }

  radiance += transport * sum_lights;

  LightSample sample = material->sample_brdf(wi, n, context.rand);

  if (sample.pdf < EPSILON) return radiance;

  float cosineterm = abs(dot(sample.wo, n));
  transport = transport * (sample.brdf * (cosineterm / sample.pdf));

  if (length2(transport) < EPSILON) return radiance;

  Ray next_ray{dot(sample.wo, n) >= 0 ? offset_up : offset_down, sample.wo};

  return incoming_light_helper(context, next_ray, radiance, transport,
                               bounce + 1);
}

vec3 incoming_light(const Context& context, const Ray& ray) {
  return incoming_light_helper(context, ray, glm::zero<vec3>(),
                               glm::one<vec3>(), 0);
}
}  // namespace

namespace trace {
void pathtrace(const KdTreeLinked& kdtree,
               const vector<SphereLight>& lights,
               const Pinhole& pinhole,
               unsigned int bounces,
               FastRand* rand,
               SampleBuffer& buffer) {
  Context context{kdtree, lights, bounces, rand};
  for (unsigned int y = 0; y < buffer.height(); ++y) {
    for (unsigned int x = 0; x < buffer.width(); ++x) {
      float sx = (static_cast<float>(x) + rand->next()) / pinhole.width;
      float sy = (static_cast<float>(y) + rand->next()) / pinhole.height;

      buffer.add(x, y, incoming_light(context, pinhole.ray(sx, sy)));
    }
  }

  buffer.inc();
}
}  // namespace trace
