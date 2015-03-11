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

    vec3 fromLight(
        const Scene& scene,
        const Material* material,
        const vec3& target,
        const vec3& offset,
        const vec3& wi,
        const vec3& n,
        const SphereLight& light,
        FastRand& rand)
    {
      const vec3 source    = lightSample(rand, light);
      const vec3 direction = source - target;

      const Ray shadow_ray { offset , direction };

      if (!scene.anyIntersection(shadow_ray, 0.0f, 1.0f)) {
        const vec3 wr = normalize(direction);

        const vec3 radiance = lightEmitted(light, target);

        return material->brdf(wi, wr, n) * radiance * abs(dot(wr, n));
      }

      return zero<vec3>();
    }

    vec3 environmentLight(const Ray&)
    {
      return vec3(0.8f, 0.8f, 0.8f);
    }

    vec3 incomingLightHelper(
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
      if (!scene.allIntersection(ray, 0.0f, FLT_MAX, isect))
        return radiance + transport * environmentLight(ray);

      const vec3 wi    = -ray.direction;
      const vec3 point = isect.position;
      const vec3 n     = isect.normal;

      const Material* material = isect.material;

      const vec3 offset     = EPSILON * n;
      const vec3 offsetUp   = point + offset;
      const vec3 offsetDown = point - offset;

      vec3 sumLights = zero<vec3>();
      for (const SphereLight& light : scene.lights()) {
        sumLights += fromLight(scene, material, point, offsetUp, wi, n, light, rand);
      }

      radiance += transport * sumLights;

      const LightSample sample = material->sample_brdf(rand, wi, n);

      if (sample.pdf < EPSILON)
        return radiance;

      const float cosineterm = abs(dot(sample.wo, n));
      transport = transport * (sample.brdf * (cosineterm / sample.pdf));

      if (length2(transport) < EPSILON)
        return radiance;

      Ray next_ray
        { dot(sample.wo, n) >= 0 ? offsetUp : offsetDown
        , sample.wo
        };

      return incomingLightHelper(
          scene,
          next_ray,
          rand,
          radiance,
          transport,
          bounce + 1);
    }

    vec3 incomingLight(
        const Scene& scene,
        const Ray& ray,
        FastRand& rand)
    {
      return incomingLightHelper(
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

    vec3 X = camera_up        * sin(radians(fov_half));
    vec3 Y = camera_right     * sin(radians(fov_half)) * aspect;
    vec3 Z = camera.direction * cos(radians(fov_half));

    m_camera_pos = camera.position;

    m_min_d = Z - Y - X;

    m_dx = 2.0f * ((Z - X) - m_min_d);
    m_dy = 2.0f * ((Z - Y) - m_min_d);
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

        const vec3 light = incomingLight(m_scene, ray, rand);
        buffer.add(x, y, light);
      }
    }

    buffer.inc();
  }
}
