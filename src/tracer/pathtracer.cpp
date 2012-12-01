#include "pathtracer.hpp"

#include "math/helpers.hpp"
#include "math/ray.hpp"

#include <glm/gtx/constants.hpp>
#include <iostream>

using namespace glm;
using namespace math;
using namespace std;

Pathtracer::Pathtracer(size_t w, size_t h, const Scene& scene)
  : m_frameBufferWidth(w), m_frameBufferHeight(h),
    m_frameBufferSamples(0), m_frameBuffer(w * h), m_scene(scene) {
}

Pathtracer::~Pathtracer() { }

// -----------------------------------------------------------------------
// Create and trace a ray per pixel
void Pathtracer::tracePrimaryRays() {
  // Scene must have a camera
  if (m_scene.m_cameras.empty()) {
    cout << "Scene has no cameras!\n";
    exit(1);
  }

  const float width  = static_cast<float>(m_frameBufferWidth);
  const float height = static_cast<float>(m_frameBufferHeight);

  // Initialize selected camera
  const Camera& camera = m_scene.m_cameras[m_selectedCamera % m_scene.m_cameras.size()];

  const vec3 camera_pos   = camera.m_position;
  const vec3 camera_dir   = camera.m_direction;
  const vec3 camera_right = normalize(cross(camera_dir, camera.m_up));
  const vec3 camera_up    = normalize(cross(camera_right, camera_dir));

  const float camera_fov         = camera.m_fov;
  const float camera_aspectRatio = width / height;

  const float camera_fov_half = camera_fov / 2.0f;

  const vec3 Z = camera_dir   * cos(radians(camera_fov_half));
  const vec3 X = camera_up    * sin(radians(camera_fov_half));
  const vec3 Y = camera_right * sin(radians(camera_fov_half))  * camera_aspectRatio;

  const vec3 min_d = Z - Y - X;

  const vec3 dX = 2.0f * ((Z - X) - min_d);
  const vec3 dY = 2.0f * ((Z - Y) - min_d);

  //#pragma omp parallel for
  for (size_t y = 0; y < m_frameBufferHeight; ++y) {
    for (size_t x = 0; x < m_frameBufferWidth; ++x) {
      const vec2 screenCoord = vec2(
          (static_cast<float>(x) + randf()) / width,
          (static_cast<float>(y) + randf()) / height
      );

      Ray primaryRay(camera_pos,
          normalize(min_d + screenCoord.x * dX + screenCoord.y * dY),
          0.0f, FLT_MAX);

      Intersection isect;
      if (m_scene.allIntersection(primaryRay, isect)) {
        m_frameBuffer[y * m_frameBufferWidth + x] += Li(primaryRay, isect);
      } else {
        m_frameBuffer[y * m_frameBufferWidth + x] += Lenvironment(primaryRay);
      }
    }
  }

  m_frameBufferSamples += 1;
}

// -----------------------------------------------------------------------
// Evaluate the outgoing radiance from the first intersection point of
// a primary ray.
vec3 Pathtracer::Li(const Ray& primaryRay, const Intersection& primaryIsect) {
  vec3 L       = zero<vec3>();
  vec3 path_tp = one<vec3>();

  Ray current_ray(primaryRay);
  Intersection isect(primaryIsect);

  for (size_t i = 0; i < PT_MAX_BOUNCES; ++i) {
    const Material& mat = *isect.m_material;
    const vec3 wi       = -current_ray.d;

    const vec3 offsetInNormalDir = PT_EPSILON * isect.m_normal;

    for (const Light& light : m_scene.m_lights) {
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
    const vec3 brdf = mat.sample_f(wi, wo, isect, pdf);

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
vec3 Pathtracer::Lenvironment(const Ray& r) {
  if (r.d.y > 0.0)
    return vec3(0.5f, 0.6f, 0.7f);
  else
    return vec3(0.05f, 0.025f, 0.001f);
}
