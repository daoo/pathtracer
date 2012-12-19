#include "material.hpp"
#include "mcsampling.hpp"

using namespace glm;

namespace
{
  float reflect(
      float r0,
      const vec3& wo,
      const vec3& n)
  {
    return r0 + (1.0f - r0) * pow(1.0f - abs(dot(wo, n)), 5.0f);
  }

  constexpr int sign(float v)
  {
    return v < 0.0f ? -1 : 1;
  }

  bool sameHemisphere(const vec3& wi, const vec3& wo, const vec3& n)
  {
    return sign(dot(wi, n)) == sign(dot(wo, n));
  }

  vec3 perpendicular(const vec3& v)
  {
    if (fabsf(v.x) < fabsf(v.y)) {
      return vec3(0.0f, -v.z, v.y);
    }

    return vec3(-v.z, 0.0f, v.x);
  }
}

DiffuseMaterial::DiffuseMaterial(const vec3& reflectance, Texture* map)
  : m_reflectance(reflectance), m_reflectance_map(map) { }

vec3 DiffuseMaterial::f(
    const vec3&,
    const vec3&,
    const vec3&) const
{
  return m_reflectance * one_over_pi<float>();
}

LightSample DiffuseMaterial::sample_f(
    FastRand& rand,
    const vec3& wi,
    const vec3& n) const
{
  vec3 tangent   = normalize(perpendicular(n));
  vec3 bitangent = cross(n, tangent);
  vec3 s         = cosineSampleHemisphere(rand);

  vec3 wo = normalize(s.x * tangent + s.y * bitangent + s.z * n);
  return { length(s), f(wi, wo, n), wo };
}

SpecularReflectionMaterial::SpecularReflectionMaterial(const vec3& reflectance)
  : m_reflectance(reflectance) { }

vec3 SpecularReflectionMaterial::f(
    const vec3&,
    const vec3&,
    const vec3&) const
{
  return zero<vec3>();
}

LightSample SpecularReflectionMaterial::sample_f(
    FastRand&,
    const vec3& wi,
    const vec3& n) const
{
  vec3 wo   = normalize(2.0f * abs(dot(wi, n)) * n - wi);
  float pdf = sameHemisphere(wi, wo, n) ? abs(dot(wo, n)) : 0.0f;
  return { pdf, m_reflectance, wo };
}

SpecularRefractionMaterial::SpecularRefractionMaterial(float ior)
  : m_ior(ior), m_refmat(one<vec3>()) { }

vec3 SpecularRefractionMaterial::f(
    const vec3&,
    const vec3&,
    const vec3&) const
{
  return zero<vec3>();
}

LightSample SpecularRefractionMaterial::sample_f(
    FastRand& rand,
    const vec3& wi,
    const vec3& n) const
{
  float eta;
  if(dot(-wi, n) < 0.0f) eta = 1.0f/m_ior;
  else eta = m_ior;

  vec3 N = dot(-wi, n) < 0.0 ? n : -n;

  float w = -dot(-wi, N) * eta;
  float k = 1.0f + (w - eta) * (w + eta);
  if (k < 0.0f) {
    // Total internal reflection
    return m_refmat.sample_f(rand, wi, N);
  }

  k       = sqrt(k);
  vec3 wo = normalize(-eta * wi + (w - k) * N);
  return { 1.0f, one<vec3>(), wo };
}

FresnelBlendMaterial::FresnelBlendMaterial(const Material* reflection, const Material* refraction, float r0)
  : m_onReflectionMaterial(reflection), m_onRefractionMaterial(refraction), m_R0(r0) { }

vec3 FresnelBlendMaterial::f(
    const vec3& wo,
    const vec3& wi,
    const vec3& n) const
{
  float _R = reflect(m_R0, wo, n);
  return _R * m_onReflectionMaterial->f(wo, wi, n) +
    (1.0f - _R) * m_onRefractionMaterial->f(wo, wi, n);
}

LightSample FresnelBlendMaterial::sample_f(
    FastRand& rand,
    const vec3& wi,
    const vec3& n) const
{
  if (rand() < reflect(m_R0, wi, n))
    return m_onReflectionMaterial->sample_f(rand, wi, n);
  else
    return m_onRefractionMaterial->sample_f(rand, wi, n);
}

BlendMaterial::BlendMaterial(const Material* first, const Material* second, float w)
  : m_firstMaterial(first), m_secondMaterial(second), m_w(w) { }

vec3 BlendMaterial::f(
    const vec3& wo,
    const vec3& wi,
    const vec3& n) const
{
  return m_w * m_firstMaterial->f(wo, wi, n) + (1.0f - m_w) * m_secondMaterial->f(wo, wi, n);
}

LightSample BlendMaterial::sample_f(
    FastRand& rand,
    const vec3& wi,
    const vec3& n) const
{
  if (rand() < m_w)
    return m_firstMaterial->sample_f(rand, wi, n);
  else
    return m_secondMaterial->sample_f(rand, wi, n);
}
