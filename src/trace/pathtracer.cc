#include "trace/pathtracer.h"

#include "kdtree/array.h"
#include "kdtree/intersection.h"
#include "kdtree/traverse.h"
#include "trace/material.h"
#include "trace/mcsampling.h"
#include <glm/gtc/constants.hpp>

using namespace glm;
using namespace std;

namespace kdtree {
bool any_intersects(const KdTreeArray& kdtree,
                    const geometry::Ray& ray,
                    float tmin,
                    float tmax) {
  Intersection isect;
  return search_tree(kdtree, ray, tmin, tmax, isect);
}
}

namespace trace {
namespace {
constexpr unsigned int MAX_BOUNCES = 16;
constexpr float EPSILON = 0.00001f;

vec3 from_light(const kdtree::KdTreeArray& kdtree,
                const Material* material,
                const vec3& target,
                const vec3& offset,
                const vec3& wi,
                const vec3& n,
                const SphereLight& light,
                FastRand& rand) {
  const vec3 source = light_sample(rand, light);
  const vec3 direction = source - target;

  const geometry::Ray shadow_ray{offset, direction};

  if (!any_intersects(kdtree, shadow_ray, 0.0f, 1.0f)) {
    const vec3 wr = normalize(direction);

    const vec3 radiance = light_emitted(light, target);

    return material->brdf(wi, wr, n) * radiance * abs(dot(wr, n));
  }

  return zero<vec3>();
}

vec3 environment_light(const geometry::Ray&) {
  return vec3(0.8f, 0.8f, 0.8f);
}

vec3 incoming_light_helper(const kdtree::KdTreeArray& kdtree,
                           const vector<SphereLight>& lights,
                           const geometry::Ray& ray,
                           FastRand& rand,
                           vec3 radiance,
                           vec3 transport,
                           size_t bounce) {
  if (bounce >= MAX_BOUNCES) return radiance;

  kdtree::Intersection isect;
  if (!kdtree::search_tree(kdtree, ray, 0.0f, FLT_MAX, isect))
    return radiance + transport * environment_light(ray);

  const vec3 wi = -ray.direction;
  const vec3 point = isect.position;
  const vec3 n = isect.normal;

  const Material* material = static_cast<const Material*>(isect.tag);

  const vec3 offset = EPSILON * n;
  const vec3 offset_up = point + offset;
  const vec3 offset_down = point - offset;

  vec3 sum_lights = zero<vec3>();
  for (const SphereLight& light : lights) {
    sum_lights +=
        from_light(kdtree, material, point, offset_up, wi, n, light, rand);
  }

  radiance += transport * sum_lights;

  const LightSample sample = material->sample_brdf(rand, wi, n);

  if (sample.pdf < EPSILON) return radiance;

  const float cosineterm = abs(dot(sample.wo, n));
  transport = transport * (sample.brdf * (cosineterm / sample.pdf));

  if (length2(transport) < EPSILON) return radiance;

  geometry::Ray next_ray{dot(sample.wo, n) >= 0 ? offset_up : offset_down,
                         sample.wo};

  return incoming_light_helper(kdtree, lights, next_ray, rand, radiance,
                               transport, bounce + 1);
}

vec3 incoming_light(const kdtree::KdTreeArray& kdtree,
                    const vector<SphereLight>& lights,
                    const geometry::Ray& ray,
                    FastRand& rand) {
  return incoming_light_helper(kdtree, lights, ray, rand, zero<vec3>(),
                               one<vec3>(), 0);
}
}

void pathtrace(const kdtree::KdTreeArray& kdtree,
               const vector<SphereLight>& lights,
               const Pinhole& pinhole,
               FastRand& rand,
               SampleBuffer& buffer) {
  for (unsigned int y = 0; y < buffer.height(); ++y) {
    for (unsigned int x = 0; x < buffer.width(); ++x) {
      float sx = (static_cast<float>(x) + rand.next()) / pinhole.width;
      float sy = (static_cast<float>(y) + rand.next()) / pinhole.height;

      buffer.add(x, y,
                 incoming_light(kdtree, lights, pinhole.ray(sx, sy), rand));
    }
  }

  buffer.inc();
}
}
