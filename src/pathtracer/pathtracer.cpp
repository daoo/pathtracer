#include "pathtracer.hpp"

#include "math/helpers.hpp"
#include "math/ray.hpp"

#include <glm/gtx/constants.hpp>

using namespace glm;
using namespace math;
using namespace std;

namespace
{
  constexpr size_t PT_MAX_BOUNCES = 16;
  constexpr float PT_EPSILON      = 0.00001f;
}

Pathtracer::Pathtracer(const Scene& scene, size_t camera_index, size_t width, size_t height)
  : m_scene(scene), m_fwidth(static_cast<float>(width)), m_fheight(static_cast<float>(height))
{
  assert(!scene.cameras().empty());

  const Camera& camera = m_scene.cameras()[camera_index % m_scene.cameras().size()];

  vec3 camera_right = normalize(cross(camera.m_direction, camera.m_up));
  vec3 camera_up    = normalize(cross(camera_right, camera.m_direction));

  float aspect   = m_fwidth / m_fheight;
  float fov_half = camera.m_fov / 2.0f;

  vec3 Z = camera.m_direction * cos(radians(fov_half));
  vec3 X = camera_up          * sin(radians(fov_half));
  vec3 Y = camera_right       * sin(radians(fov_half)) * aspect;

  m_camera_pos = camera.m_position;

  m_min_d = Z - Y - X;

  m_dx = 2.0f * ((Z - X) - m_min_d);
  m_dy = 2.0f * ((Z - Y) - m_min_d);
}

Pathtracer::~Pathtracer() { }

void Pathtracer::tracePrimaryRays(FastRand& rand, SampleBuffer& buffer) const
{
  float fw = static_cast<float>(buffer.width());
  float fh = static_cast<float>(buffer.height());

  for (size_t y = 0; y < buffer.height(); ++y) {
    for (size_t x = 0; x < buffer.width(); ++x) {
      const vec2 screenCoord = vec2(
          (static_cast<float>(x) + rand()) / fw,
          (static_cast<float>(y) + rand()) / fh
      );

      Ray primaryRay
        { m_camera_pos
        , normalize(m_min_d + screenCoord.x * m_dx + screenCoord.y * m_dy)
        , 0.0f
        , FLT_MAX
        };

      Intersection isect;
      if (m_scene.allIntersection(primaryRay, isect)) {
        buffer.add(x, y, incomingLight(rand, primaryRay, isect));
      } else {
        buffer.add(x, y, environmentLight(primaryRay));
      }
    }
  }

  buffer.increaseSamples();
}

vec3 Pathtracer::incomingLight(
    FastRand& rand,
    const Ray& primaryRay,
    const Intersection& primaryIsect) const
{
  vec3 L       = zero<vec3>();
  vec3 path_tp = one<vec3>();

  Ray current_ray(primaryRay);
  Intersection isect(primaryIsect);

  for (size_t i = 0; i < PT_MAX_BOUNCES; ++i) {
    const Material* mat = isect.m_material;
    const vec3 wi       = -current_ray.direction;

    const vec3 offsetInNormalDir = PT_EPSILON * isect.m_normal;

    for (const SphereLight& light : m_scene.lights()) {
      const vec3 isectPosition    = isect.m_position + offsetInNormalDir;
      const vec3 lightSamplePos   = lightSample(light);
      const vec3 directionToLight = lightSamplePos - isectPosition;

      const Ray shadow_ray
        { isectPosition
        , directionToLight
        , 0.0f
        , 1.0f
        };

      if (!m_scene.anyIntersection(shadow_ray)) {
        const vec3 wo = normalize(directionToLight);
        const vec3 li = lightEmitted(light, isect.m_position);

        L += path_tp
           * mat->brdf(wi, wo, isect.m_normal)
           * li
           * abs(dot(wo, isect.m_normal));
      }
    }

    const LightSample sample = mat->sample_brdf(rand, wi, isect.m_normal);

    if (sample.pdf < PT_EPSILON) {
      return L;
    }

    const float cosineterm = abs(dot(sample.wo, isect.m_normal));
    path_tp = path_tp * (sample.brdf * (cosineterm / sample.pdf));

    if (lengthSquared(path_tp) < PT_EPSILON * PT_EPSILON) {
      return L;
    }

    if (dot(sample.wo, isect.m_normal) >= 0) {
      current_ray = Ray { isect.m_position + offsetInNormalDir, sample.wo, 0.0f, FLT_MAX };
    } else {
      current_ray = Ray { isect.m_position - offsetInNormalDir, sample.wo, 0.0f, FLT_MAX };
    }

    if (!m_scene.allIntersection(current_ray, isect)) {
      return L + path_tp * environmentLight(current_ray);
    }
  }

  return L;
}

// -----------------------------------------------------------------------
// Evaluate the outgoing radiance from the environment
vec3 Pathtracer::environmentLight(const Ray&) const
{
  return vec3(0.8f, 0.8f, 0.8f);
}
