#include "pathtracer.hpp"

#include "math/helpers.hpp"
#include "math/ray.hpp"

#include <glm/gtx/constants.hpp>

using namespace glm;
using namespace math;
using namespace std;

namespace {
  constexpr size_t PT_MAX_BOUNCES = 16;
  constexpr float PT_EPSILON      = 0.00001f;
}

Pathtracer::Pathtracer(size_t w, size_t h, const Scene& scene, size_t camera_index)
  : m_buffer(w * h),
    m_samples(0),

    m_iwidth(w), m_iheight(h),
    m_fwidth(static_cast<float>(w)), m_fheight(static_cast<float>(h)),

    m_scene(scene),
    m_fastrand() {
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

void Pathtracer::tracePrimaryRays() {
  for (size_t y = 0; y < m_iheight; ++y) {
    for (size_t x = 0; x < m_iwidth; ++x) {
      const vec2 screenCoord = vec2(
          (static_cast<float>(x) + m_fastrand()) / m_fwidth,
          (static_cast<float>(y) + m_fastrand()) / m_fheight
      );

      Ray primaryRay(m_camera_pos,
          normalize(m_min_d + screenCoord.x * m_dx + screenCoord.y * m_dy),
          0.0f, FLT_MAX);

      Intersection isect;
      if (m_scene.allIntersection(primaryRay, isect)) {
        m_buffer[y * m_iwidth + x] += Li(primaryRay, isect);
      } else {
        m_buffer[y * m_iwidth + x] += Lenvironment(primaryRay);
      }
    }
  }

  m_samples += 1;
}

vec3 Pathtracer::Li(const Ray& primaryRay, const Intersection& primaryIsect) {
  vec3 L       = zero<vec3>();
  vec3 path_tp = one<vec3>();

  Ray current_ray(primaryRay);
  Intersection isect(primaryIsect);

  for (size_t i = 0; i < PT_MAX_BOUNCES; ++i) {
    const Material& mat = *isect.m_material;
    const vec3 wi       = -current_ray.direction;

    const vec3 offsetInNormalDir = PT_EPSILON * isect.m_normal;

    for (const Light& light : m_scene.lights()) {
      const vec3 isectPosition    = isect.m_position + offsetInNormalDir;
      const vec3 lightSamplePos   = sample(light);
      const vec3 directionToLight = lightSamplePos - isectPosition;

      const Ray shadow_ray(isectPosition, directionToLight, 0.0f, 1.0f);
      if (!m_scene.anyIntersection(shadow_ray)) {
        const vec3 wo = normalize(directionToLight);
        const vec3 li = Le(light, isect.m_position);

        L += path_tp * mat.f(wi, wo, isect) * li * abs(dot(wo, isect.m_normal));
      }
    }

    float pdf;
    vec3 wo;
    const vec3 brdf = mat.sample_f(m_fastrand, wi, wo, isect, pdf);

    if (pdf < PT_EPSILON) {
      return L;
    }

    const float cosineterm = abs(dot(wo, isect.m_normal));
    path_tp = path_tp * (brdf * (cosineterm / pdf));

    if (lengthSquared(path_tp) < PT_EPSILON * PT_EPSILON) {
      return L;
    }

    if (dot(wo, isect.m_normal) >= 0) {
      current_ray = Ray(isect.m_position + offsetInNormalDir, wo, 0.0f, FLT_MAX);
    } else {
      current_ray = Ray(isect.m_position - offsetInNormalDir, wo, 0.0f, FLT_MAX);
    }

    if (!m_scene.allIntersection(current_ray, isect)) {
      return L + path_tp * Lenvironment(current_ray);
    }
  }

  return L;
}

// -----------------------------------------------------------------------
// Evaluate the outgoing radiance from the environment
vec3 Pathtracer::Lenvironment(const Ray& ray) {
  if (ray.direction.y > 0.0)
    return vec3(0.5f, 0.6f, 0.7f);
  else
    return vec3(0.05f, 0.025f, 0.001f);
}
