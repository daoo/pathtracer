#include "trace/material.h"

#include "trace/fastrand.h"
#include "trace/mcsampling.h"

using glm::vec3;

namespace trace {
namespace {
float reflectance(float r0, const vec3& wo, const vec3& n) {
  return r0 +
         (1.0f - r0) * glm::pow(1.0f - glm::abs<float>(glm::dot(wo, n)), 5.0f);
}

constexpr int same_sign(float a, float b) {
  return (a < 0 && b < 0) || (a >= 0 && b >= 0);
}

bool same_hemisphere(const vec3& wi, const vec3& wo, const vec3& n) {
  return same_sign(glm::dot(wi, n), glm::dot(wo, n));
}

vec3 perpendicular(const vec3& v) {
  if (glm::abs<float>(v.x) < glm::abs<float>(v.y)) {
    return vec3(0.0f, -v.z, v.y);
  }

  return vec3(-v.z, 0.0f, v.x);
}
}  // namespace

Material::~Material() {}

DiffuseMaterial::DiffuseMaterial(const vec3& reflectance)
    : reflectance_(reflectance) {}

vec3 DiffuseMaterial::brdf(const vec3&, const vec3&, const vec3&) const {
  return reflectance_ * glm::one_over_pi<float>();
}

LightSample DiffuseMaterial::sample_brdf(const vec3& wi,
                                         const vec3& n,
                                         FastRand* rand) const {
  vec3 tangent = normalize(perpendicular(n));
  vec3 bitangent = cross(n, tangent);
  vec3 s = cosine_sample_hemisphere(rand);

  vec3 wo = normalize(s.x * tangent + s.y * bitangent + s.z * n);
  return {length(s), brdf(wi, wo, n), wo};
}

SpecularReflectionMaterial::SpecularReflectionMaterial(const vec3& reflectance)
    : reflectance_(reflectance) {}

vec3 SpecularReflectionMaterial::brdf(const vec3&,
                                      const vec3&,
                                      const vec3&) const {
  return glm::zero<vec3>();
}

LightSample SpecularReflectionMaterial::sample_brdf(const vec3& wi,
                                                    const vec3& n,
                                                    FastRand*) const {
  vec3 wo = normalize(2.0f * glm::abs<float>(glm::dot(wi, n)) * n - wi);
  float pdf =
      same_hemisphere(wi, wo, n) ? glm::abs<float>(glm::dot(wo, n)) : 0.0f;
  return {pdf, reflectance_, wo};
}

SpecularRefractionMaterial::SpecularRefractionMaterial(float ior)
    : index_of_refraction_(ior), specular_reflection_(glm::one<vec3>()) {}

vec3 SpecularRefractionMaterial::brdf(const vec3&,
                                      const vec3&,
                                      const vec3&) const {
  return glm::zero<vec3>();
}

LightSample SpecularRefractionMaterial::sample_brdf(const vec3& wi,
                                                    const vec3& n,
                                                    FastRand* rand) const {
  float a = glm::dot(-wi, n);
  float eta = a < 0.0f ? 1.0f / index_of_refraction_ : index_of_refraction_;
  vec3 N = a < 0.0f ? n : -n;

  float w = -a * eta;
  float k = 1.0f + (w - eta) * (w + eta);

  if (k < 0.0f) {
    // Total internal reflection
    return specular_reflection_.sample_brdf(wi, N, rand);
  }

  k = glm::sqrt(k);
  vec3 wo = glm::normalize(-eta * wi + (w - k) * N);
  return {1.0f, glm::one<vec3>(), wo};
}

FresnelBlendMaterial::FresnelBlendMaterial(const Material* reflection,
                                           const Material* refraction,
                                           float r0)
    : reflection_(reflection), refraction_(refraction), r0_(r0) {}

FresnelBlendMaterial::~FresnelBlendMaterial() {
  delete reflection_;
  delete refraction_;
}

vec3 FresnelBlendMaterial::brdf(const vec3& wo,
                                const vec3& wi,
                                const vec3& n) const {
  return glm::mix(refraction_->brdf(wo, wi, n), reflection_->brdf(wo, wi, n),
                  reflectance(r0_, wo, n));
}

LightSample FresnelBlendMaterial::sample_brdf(const vec3& wi,
                                              const vec3& n,
                                              FastRand* rand) const {
  return rand->next() < reflectance(r0_, wi, n)
             ? reflection_->sample_brdf(wi, n, rand)
             : refraction_->sample_brdf(wi, n, rand);
}

BlendMaterial::BlendMaterial(const Material* first,
                             const Material* second,
                             float w)
    : first_(first), second_(second), factor_(w) {}

BlendMaterial::~BlendMaterial() {
  delete first_;
  delete second_;
}

vec3 BlendMaterial::brdf(const vec3& wo, const vec3& wi, const vec3& n) const {
  return glm::mix(second_->brdf(wo, wi, n), first_->brdf(wo, wi, n), factor_);
}

LightSample BlendMaterial::sample_brdf(const vec3& wi,
                                       const vec3& n,
                                       FastRand* rand) const {
  return rand->next() < factor_ ? first_->sample_brdf(wi, n, rand)
                                : second_->sample_brdf(wi, n, rand);
}
}  // namespace trace
