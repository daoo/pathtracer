#ifndef TRACE_MATERIAL_H_
#define TRACE_MATERIAL_H_

#include <glm/glm.hpp>

#include "trace/texture.h"

namespace trace {
class FastRand;

struct LightSample {
  float pdf;
  glm::vec3 brdf;
  glm::vec3 wo;
};

/**
 * Abstract base class for materials.
 */
class Material {
 public:
  virtual ~Material() = 0;

  virtual glm::vec3 brdf(const glm::vec3&,
                         const glm::vec3&,
                         const glm::vec3&) const = 0;

  virtual LightSample sample_brdf(const glm::vec3&,
                                  const glm::vec3&,
                                  FastRand*) const = 0;
};

/**
 * Diffuse reflection
 *
 * The BRDF for diffuse reflection is constant. It can be importance sampled
 * on the cosine term to improve sampling quality somewhat.
 */
class DiffuseMaterial final : public Material {
 public:
  explicit DiffuseMaterial(const glm::vec3&);

  virtual glm::vec3 brdf(const glm::vec3&,
                         const glm::vec3&,
                         const glm::vec3&) const;

  virtual LightSample sample_brdf(const glm::vec3&,
                                  const glm::vec3&,
                                  FastRand*) const;

 private:
  glm::vec3 reflectance_;
};

/**
 * Textured diffuse material.
 */
class DiffuseTextureMaterial final : public Material {
 public:
  DiffuseTextureMaterial(const glm::vec3&, const Texture&);

  virtual glm::vec3 brdf(const glm::vec3&,
                         const glm::vec3&,
                         const glm::vec3&) const;

  virtual LightSample sample_brdf(const glm::vec3&,
                                  const glm::vec3&,
                                  FastRand*) const;

 private:
  glm::vec3 reflectance_;
  Texture texture_;
};

/**
 * Perfect specular reflection
 *
 * This BRDF will allways sample EXACTLY one direction given a normal and an
 * incident vector. Therefore, pdf = 1.0 and f can just be set to
 * zero as we will never select a light sample which is exactly in that
 * direction.
 */
class SpecularReflectionMaterial final : public Material {
 public:
  explicit SpecularReflectionMaterial(const glm::vec3&);

  virtual glm::vec3 brdf(const glm::vec3&,
                         const glm::vec3&,
                         const glm::vec3&) const;

  virtual LightSample sample_brdf(const glm::vec3&,
                                  const glm::vec3&,
                                  FastRand*) const;

 private:
  // The reflectance (color) of the specular reflection
  glm::vec3 reflectance_;
};

/**
 * Perfect specular refraction
 *
 * This BTDF oll allways sample EXACTLY one direction given a normal,
 * incident vector and index of refraction. Therefore, pdf = 1.0 and f can
 * just be set to zero as we oll never select a light sample which is exactly
 * in that direction.
 */
class SpecularRefractionMaterial final : public Material {
 public:
  explicit SpecularRefractionMaterial(float);

  virtual glm::vec3 brdf(const glm::vec3&,
                         const glm::vec3&,
                         const glm::vec3&) const;

  virtual LightSample sample_brdf(const glm::vec3&,
                                  const glm::vec3&,
                                  FastRand*) const;

 private:
  float index_of_refraction_;
  SpecularReflectionMaterial specular_reflection_;
};

/**
 * Fresnel blending
 *
 * This Material actually combines two brdfs of a view dependent fresnel
 * term. We use the Schlick approximation to the real fresnel equations,
 * which irks quite well for conductors.
 */
class FresnelBlendMaterial final : public Material {
 public:
  FresnelBlendMaterial(const Material*, const Material*, float);
  ~FresnelBlendMaterial();

  virtual glm::vec3 brdf(const glm::vec3&,
                         const glm::vec3&,
                         const glm::vec3&) const;

  virtual LightSample sample_brdf(const glm::vec3&,
                                  const glm::vec3&,
                                  FastRand*) const;

 private:
  FresnelBlendMaterial(const FresnelBlendMaterial&) = delete;
  FresnelBlendMaterial& operator=(const FresnelBlendMaterial&) = delete;

  const Material* reflection_;
  const Material* refraction_;
  float r0_;
};

/**
 * Linear blending
 *
 * This Material combines two brdfs linearly as
 *
 *   w * M1 + (1 - w) * M2
 */
class BlendMaterial final : public Material {
 public:
  BlendMaterial(const Material*, const Material*, float);
  ~BlendMaterial();

  virtual glm::vec3 brdf(const glm::vec3&,
                         const glm::vec3&,
                         const glm::vec3&) const;

  virtual LightSample sample_brdf(const glm::vec3&,
                                  const glm::vec3&,
                                  FastRand*) const;

 private:
  BlendMaterial(const BlendMaterial&) = delete;
  BlendMaterial& operator=(const BlendMaterial&) = delete;

  const Material* first_;
  const Material* second_;
  float factor_;
};
}  // namespace trace

#endif  // TRACE_MATERIAL_H_
