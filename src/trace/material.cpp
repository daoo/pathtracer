#include "material.hpp"
#include "mcsampling.hpp"

using namespace glm;

namespace trace {
namespace {
float reflectance(float r0, const vec3& wo, const vec3& n) {
  return r0 + (1.0f - r0) * pow(1.0f - glm::abs<float>(dot(wo, n)), 5.0f);
}

vec3 blend(float w, const vec3& a, const vec3& b) {
  return w * a + (1.0f - w) * b;
}

constexpr int same_sign(float a, float b) {
  return (a < 0 && b < 0) || (a >= 0 && b >= 0);
}

bool same_hemisphere(const vec3& wi, const vec3& wo, const vec3& n) {
  return same_sign(dot(wi, n), dot(wo, n));
}

vec3 perpendicular(const vec3& v) {
  if (glm::abs<float>(v.x) < glm::abs<float>(v.y)) {
    return vec3(0.0f, -v.z, v.y);
  }

  return vec3(-v.z, 0.0f, v.x);
}
}

Material::~Material() {}

DiffuseMaterial::DiffuseMaterial(const vec3& reflectance)
    : m_reflectance(reflectance) {}

vec3 DiffuseMaterial::brdf(const vec3&, const vec3&, const vec3&) const {
  return m_reflectance * one_over_pi<float>();
}

LightSample DiffuseMaterial::sample_brdf(FastRand& rand,
                                         const vec3& wi,
                                         const vec3& n) const {
  vec3 tangent = normalize(perpendicular(n));
  vec3 bitangent = cross(n, tangent);
  vec3 s = cosine_sample_hemisphere(rand);

  vec3 wo = normalize(s.x * tangent + s.y * bitangent + s.z * n);
  return {length(s), brdf(wi, wo, n), wo};
}

SpecularReflectionMaterial::SpecularReflectionMaterial(const vec3& reflectance)
    : m_reflectance(reflectance) {}

vec3 SpecularReflectionMaterial::brdf(const vec3&,
                                      const vec3&,
                                      const vec3&) const {
  return zero<vec3>();
}

LightSample SpecularReflectionMaterial::sample_brdf(FastRand&,
                                                    const vec3& wi,
                                                    const vec3& n) const {
  vec3 wo = normalize(2.0f * glm::abs<float>(dot(wi, n)) * n - wi);
  float pdf = same_hemisphere(wi, wo, n) ? glm::abs<float>(dot(wo, n)) : 0.0f;
  return {pdf, m_reflectance, wo};
}

SpecularRefractionMaterial::SpecularRefractionMaterial(float ior)
    : m_ior(ior), m_refmat(one<vec3>()) {}

vec3 SpecularRefractionMaterial::brdf(const vec3&,
                                      const vec3&,
                                      const vec3&) const {
  return zero<vec3>();
}

LightSample SpecularRefractionMaterial::sample_brdf(FastRand& rand,
                                                    const vec3& wi,
                                                    const vec3& n) const {
  float a = dot(-wi, n);
  float eta = a < 0.0f ? 1.0f / m_ior : m_ior;
  vec3 N = a < 0.0f ? n : -n;

  float w = -a * eta;
  float k = 1.0f + (w - eta) * (w + eta);

  if (k < 0.0f) {
    // Total internal reflection
    return m_refmat.sample_brdf(rand, wi, N);
  }

  k = sqrt(k);
  vec3 wo = normalize(-eta * wi + (w - k) * N);
  return {1.0f, one<vec3>(), wo};
}

FresnelBlendMaterial::FresnelBlendMaterial(const Material* reflection,
                                           const Material* refraction,
                                           float r0)
    : m_on_reflection_material(reflection),
      m_on_refraction_material(refraction),
      m_r0(r0) {}

FresnelBlendMaterial::~FresnelBlendMaterial() {
  delete m_on_reflection_material;
  delete m_on_refraction_material;
}

vec3 FresnelBlendMaterial::brdf(const vec3& wo,
                                const vec3& wi,
                                const vec3& n) const {
  float _r = reflectance(m_r0, wo, n);
  return _r * m_on_reflection_material->brdf(wo, wi, n) +
         (1.0f - _r) * m_on_refraction_material->brdf(wo, wi, n);
}

LightSample FresnelBlendMaterial::sample_brdf(FastRand& rand,
                                              const vec3& wi,
                                              const vec3& n) const {
  if (rand.next() < reflectance(m_r0, wi, n))
    return m_on_reflection_material->sample_brdf(rand, wi, n);
  else
    return m_on_refraction_material->sample_brdf(rand, wi, n);
}

BlendMaterial::BlendMaterial(const Material* first,
                             const Material* second,
                             float w)
    : m_first_material(first), m_second_material(second), m_w(w) {}

BlendMaterial::~BlendMaterial() {
  delete m_first_material;
  delete m_second_material;
}

vec3 BlendMaterial::brdf(const vec3& wo, const vec3& wi, const vec3& n) const {
  return blend(m_w, m_first_material->brdf(wo, wi, n),
               m_second_material->brdf(wo, wi, n));
}

LightSample BlendMaterial::sample_brdf(FastRand& rand,
                                       const vec3& wi,
                                       const vec3& n) const {
  if (rand.next() < m_w)
    return m_first_material->sample_brdf(rand, wi, n);
  else
    return m_second_material->sample_brdf(rand, wi, n);
}
}
