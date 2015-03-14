#include "trace/pathtracer.hpp"

#include "trace/geometry/ray.hpp"
#include "trace/mcsampling.hpp"
#include <glm/gtc/constants.hpp>

using namespace glm;
using namespace math;
using namespace std;
namespace trace
{
  namespace
  {
    constexpr unsigned int MAX_BOUNCES = 16;
    constexpr float EPSILON            = 0.00001f;

    vec3 from_light(
        const Scene& scene,
        const Material* material,
        const vec3& target,
        const vec3& offset,
        const vec3& wi,
        const vec3& n,
        const SphereLight& light,
        FastRand& rand)
    {
      const vec3 source    = light_sample(rand, light);
      const vec3 direction = source - target;

      const Ray shadow_ray { offset , direction };

      if (!scene.any_intersection(shadow_ray, 0.0f, 1.0f)) {
        const vec3 wr = normalize(direction);

        const vec3 radiance = light_emitted(light, target);

        return material->brdf(wi, wr, n) * radiance * abs(dot(wr, n));
      }

      return zero<vec3>();
    }

    vec3 environment_light(const Ray&)
    {
      return vec3(0.8f, 0.8f, 0.8f);
    }

    vec3 incoming_light_helper(
        const Scene& scene,
        const Ray& ray,
        FastRand& rand,
        vec3 radiance,
        vec3 transport,
        int bounce)
    {
      if (bounce >= MAX_BOUNCES)
        return radiance;

      Intersection isect;
      if (!scene.all_intersection(ray, 0.0f, FLT_MAX, isect))
        return radiance + transport * environment_light(ray);

      const vec3 wi    = -ray.direction;
      const vec3 point = isect.position;
      const vec3 n     = isect.normal;

      const Material* material = isect.material;

      const vec3 offset      = EPSILON * n;
      const vec3 offset_up   = point + offset;
      const vec3 offset_down = point - offset;

      vec3 sum_lights = zero<vec3>();
      for (const SphereLight& light : scene.lights()) {
        sum_lights += from_light(scene, material, point, offset_up, wi, n, light, rand);
      }

      radiance += transport * sum_lights;

      const LightSample sample = material->sample_brdf(rand, wi, n);

      if (sample.pdf < EPSILON)
        return radiance;

      const float cosineterm = abs(dot(sample.wo, n));
      transport = transport * (sample.brdf * (cosineterm / sample.pdf));

      if (length2(transport) < EPSILON)
        return radiance;

      Ray next_ray
        { dot(sample.wo, n) >= 0 ? offset_up : offset_down
        , sample.wo
        };

      return incoming_light_helper(
          scene,
          next_ray,
          rand,
          radiance,
          transport,
          bounce + 1);
    }

    vec3 incoming_light(
        const Scene& scene,
        const Ray& ray,
        FastRand& rand)
    {
      return incoming_light_helper(
          scene,
          ray,
          rand,
          zero<vec3>(),
          one<vec3>(),
          0);
    }
  }

  Pathtracer::Pathtracer(
      const Scene& scene,
      unsigned int camera_index,
      unsigned int width,
      unsigned int height)
    : m_scene(scene)
    , m_fwidth(static_cast<float>(width))
    , m_fheight(static_cast<float>(height))
  {
    assert(!scene.cameras().empty());

    const Camera& camera = m_scene.cameras()[camera_index % m_scene.cameras().size()];

    vec3 camera_right = normalize(cross(camera.direction, camera.up));
    vec3 camera_up    = normalize(cross(camera_right, camera.direction));

    float aspect   = m_fwidth / m_fheight;
    float fov_half = camera.fov / 2.0f;

    vec3 x = camera_up        * sin(radians(fov_half));
    vec3 y = camera_right     * sin(radians(fov_half)) * aspect;
    vec3 z = camera.direction * cos(radians(fov_half));

    m_camera_pos = camera.position;

    m_min_d = z - y - x;

    m_dx = 2.0f * ((z - x) - m_min_d);
    m_dy = 2.0f * ((z - y) - m_min_d);
  }

  Pathtracer::~Pathtracer() { }

  void Pathtracer::trace(FastRand& rand, SampleBuffer& buffer) const
  {
    for (unsigned int y = 0; y < buffer.height(); ++y) {
      for (unsigned int x = 0; x < buffer.width(); ++x) {
        float sx = (static_cast<float>(x) + rand.next()) / m_fwidth;
        float sy = (static_cast<float>(y) + rand.next()) / m_fheight;

        Ray ray
          { m_camera_pos
          , normalize(m_min_d + sx * m_dx + sy * m_dy)
          };

        const vec3 light = incoming_light(m_scene, ray, rand);
        buffer.add(x, y, light);
      }
    }

    buffer.inc();
  }
}
