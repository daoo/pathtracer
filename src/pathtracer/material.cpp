#include "material.hpp"
#include "mcsampling.hpp"

using namespace glm;

namespace {
  constexpr int sign(float v) {
    return v < 0.0f ? -1 : 1;
  }

  bool sameHemisphere(const vec3& i, const vec3& o, const vec3& n) {
    return sign(dot(o, n)) == sign(dot(i, n));
  }

  vec3 perpendicular(const vec3& v) {
    if (fabsf(v.x) < fabsf(v.y)) {
      return vec3(0.0f, -v.z, v.y);
    }

    return vec3(-v.z, 0.0f, v.x);
  }
}

vec3 DiffuseMaterial::f(
    const vec3&,
    const vec3&,
    const Intersection&) const
{
  return m_reflectance * one_over_pi<float>();
}

vec3 DiffuseMaterial::sample_f(
    FastRand& rand,
    const vec3& i,
    vec3& o,
    const Intersection& isect,
    float& pdf) const
{
  const vec3& n = isect.m_normal;

  vec3 tangent   = normalize(perpendicular(n));
  vec3 bitangent = cross(n, tangent);
  vec3 s         = cosineSampleHemisphere(rand);

  o   = normalize(s.x * tangent + s.y * bitangent + s.z * n);
  pdf = length(s);
  return f(i, o, isect);
}

vec3 SpecularReflectionMaterial::f(
    const vec3&,
    const vec3&,
    const Intersection&) const
{
  return zero<vec3>();
}

vec3 SpecularReflectionMaterial::sample_f(
    FastRand&,
    const vec3& i,
    vec3& o,
    const Intersection& isect,
    float& pdf) const
{
  const vec3& n = isect.m_normal;

  o   = normalize(2.0f * abs(dot(i, n)) * n - i);
  pdf = sameHemisphere(i, o, n) ? abs(dot(o, n)) : 0.0f;
  return m_reflectance;
}

vec3 SpecularRefractionMaterial::f(
    const vec3&,
    const vec3&,
    const Intersection&) const
{
  return zero<vec3>();
}

vec3 SpecularRefractionMaterial::sample_f(
    FastRand& rand,
    const vec3& i,
    vec3& o,
    const Intersection& isect,
    float& pdf) const
{
  const vec3& n = isect.m_normal;
  float eta;
  if(dot(-i, n) < 0.0f) eta = 1.0f/m_ior;
  else eta = m_ior;

  vec3 N = dot(-i, n) < 0.0 ? n : -n;

  float w = -dot(-i,N) * eta;
  float k = 1.0f + (w-eta)*(w+eta);
  if (k < 0.0f) {
    // Total internal reflection
    SpecularReflectionMaterial refMat;
    refMat.m_reflectance = one<vec3>();
    Intersection newIntersection = isect;
    newIntersection.m_normal = N;
    return refMat.sample_f(rand, i, o, newIntersection, pdf);
  }

  k   = sqrt(k);
  o   = normalize(-eta*i + (w-k)*N);
  pdf = 1.0;
  return one<vec3>();
}

float FresnelBlendMaterial::R(
    const vec3& wo,
    const vec3& n) const {
  return m_R0 + (1.0f - m_R0) * pow(1.0f - abs(dot(wo,n)), 5.0f);
}

vec3 FresnelBlendMaterial::f(
    const vec3& wo,
    const vec3& wi,
    const Intersection& isect) const
{
  const vec3& n = isect.m_normal;
  float _R = R(wo, n);
  return _R * m_onReflectionMaterial->f(wo, wi, isect) +
    (1.0f - _R) * m_onRefractionMaterial->f(wo, wi, isect);
}

vec3 FresnelBlendMaterial::sample_f(
    FastRand& rand,
    const vec3& wo,
    vec3& wi,
    const Intersection& isect,
    float& pdf) const
{
  const vec3& n = isect.m_normal;
  if (rand() < R(wo, n))
    return m_onReflectionMaterial->sample_f(rand, wo, wi, isect, pdf);
  else
    return m_onRefractionMaterial->sample_f(rand, wo, wi, isect, pdf);
}

vec3 BlendMaterial::f(
    const vec3& wo,
    const vec3& wi,
    const Intersection& isect) const
{
  return m_w * m_firstMaterial->f(wo, wi, isect) + (1.0f - m_w) * m_secondMaterial->f(wo, wi, isect);
}

vec3 BlendMaterial::sample_f(
    FastRand& rand,
    const vec3& wo,
    vec3& wi,
    const Intersection& isect,
    float& pdf) const
{
  if (rand() < m_w)
    return m_firstMaterial->sample_f(rand, wo, wi, isect, pdf);
  else
    return m_secondMaterial->sample_f(rand, wo, wi, isect, pdf);
}
