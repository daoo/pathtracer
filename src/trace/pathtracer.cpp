#include "trace/pathtracer.hpp"

#include "math/helpers.hpp"
#include "math/ray.hpp"
#include <glm/gtc/constants.hpp>

using namespace glm;
using namespace math;
using namespace std;
namespace trace
{
  namespace
  {
    constexpr unsigned int MAX_BOUNCES = 16;
    constexpr float EPSILON            = 0.0001f;
  }

  Pathtracer::Pathtracer(const Scene& scene, unsigned int camera_index, unsigned int width, unsigned int height)
    : m_scene(scene), m_fwidth(static_cast<float>(width)), m_fheight(static_cast<float>(height))
  {
    assert(!scene.cameras().empty());

    const Camera& camera = m_scene.cameras()[camera_index % m_scene.cameras().size()];

    vec3 camera_right = normalize(cross(camera.direction, camera.up));
    vec3 camera_up    = normalize(cross(camera_right, camera.direction));

    float aspect   = m_fwidth / m_fheight;
    float fov_half = camera.fov / 2.0f;

    vec3 Z = camera.direction * cos(radians(fov_half));
    vec3 X = camera_up        * sin(radians(fov_half));
    vec3 Y = camera_right     * sin(radians(fov_half)) * aspect;

    m_camera_pos = camera.position;

    m_min_d = Z - Y - X;

    m_dx = 2.0f * ((Z - X) - m_min_d);
    m_dy = 2.0f * ((Z - Y) - m_min_d);
  }

  Pathtracer::~Pathtracer() { }

  void Pathtracer::tracePrimaryRays(FastRand& rand, SampleBuffer& buffer) const
  {
    float fw = static_cast<float>(buffer.width());
    float fh = static_cast<float>(buffer.height());

    for (unsigned int y = 0; y < buffer.height(); ++y) {
      for (unsigned int x = 0; x < buffer.width(); ++x) {
        vec2 screenCoord
          { (static_cast<float>(x) + rand()) / fw
          , (static_cast<float>(y) + rand()) / fh
          };

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

    for (unsigned int i = 0; i < MAX_BOUNCES; ++i) {
      const Material* mat = isect.material;
      const vec3 wi       = -current_ray.direction;

      const vec3 offsetInNormalDir = EPSILON * isect.normal;

      for (const SphereLight& light : m_scene.lights()) {
        const vec3 isectPosition    = isect.position + offsetInNormalDir;
        const vec3 lightSamplePos   = lightSample(rand, light);
        const vec3 directionToLight = lightSamplePos - isectPosition;

        const Ray shadow_ray
        { isectPosition
          , directionToLight
            , 0.0f
            , 1.0f
        };

        if (!m_scene.anyIntersection(shadow_ray)) {
          const vec3 wo = normalize(directionToLight);
          const vec3 li = lightEmitted(light, isect.position);

          L += path_tp
            * mat->brdf(wi, wo, isect.normal)
            * li
            * abs(dot(wo, isect.normal));
        }
      }

      const LightSample sample = mat->sample_brdf(rand, wi, isect.normal);

      if (sample.pdf < EPSILON) {
        return L;
      }

      const float cosineterm = abs(dot(sample.wo, isect.normal));
      path_tp = path_tp * (sample.brdf * (cosineterm / sample.pdf));

      if (lengthSquared(path_tp) < EPSILON * EPSILON) {
        return L;
      }

      if (dot(sample.wo, isect.normal) >= 0) {
        current_ray = Ray { isect.position + offsetInNormalDir, sample.wo, 0.0f, FLT_MAX };
      } else {
        current_ray = Ray { isect.position - offsetInNormalDir, sample.wo, 0.0f, FLT_MAX };
      }

      if (!m_scene.allIntersection(current_ray, isect)) {
        return L + path_tp * environmentLight(current_ray);
      }
    }

    return L;
  }

  /**
   * Evaluate the outgoing radiance from the environment.
   */
  vec3 Pathtracer::environmentLight(const Ray&) const
  {
    return vec3(0.8f, 0.8f, 0.8f);
  }
}
