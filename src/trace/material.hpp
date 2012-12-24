#ifndef MATERIAL_HPP_FNROXKUG
#define MATERIAL_HPP_FNROXKUG

#include "fastrand.hpp"
#include <glm/glm.hpp>

namespace trace
{
  struct LightSample
  {
    float pdf;
    glm::vec3 brdf;
    glm::vec3 wo;
  };

  /**
   * Abstract base class for materials.
   */
  class Material
  {
    public:
      virtual ~Material() { }

      virtual glm::vec3 brdf(
          const glm::vec3& wo,
          const glm::vec3& wi,
          const glm::vec3& normal) const = 0;

      virtual LightSample sample_brdf(
          FastRand&,
          const glm::vec3& wi,
          const glm::vec3& normal) const = 0;
  };

  /**
   * Diffuse reflection
   *
   * The BRDF for diffuse reflection is constant. It can be importance sampled
   * on the cosine term to improve sampling quality somewhat.
   */
  class DiffuseMaterial : public Material
  {
    public:
      DiffuseMaterial(const glm::vec3&);

      virtual glm::vec3 brdf(
          const glm::vec3&,
          const glm::vec3&,
          const glm::vec3&) const;

      virtual LightSample sample_brdf(
          FastRand&,
          const glm::vec3&,
          const glm::vec3&) const;

    private:
      glm::vec3 m_reflectance;
  };

  /**
   * Perfect specular reflection
   *
   * This BRDF will allways sample EXACTLY one direction given a normal and an
   * incident vector. Therefore, pdf = 1.0 and f can just be set to
   * zero as we will never select a light sample which is exactly in that
   * direction.
   */
  class SpecularReflectionMaterial : public Material
  {
    public:
      SpecularReflectionMaterial(const glm::vec3&);

      virtual glm::vec3 brdf(
          const glm::vec3&,
          const glm::vec3&,
          const glm::vec3&) const;

      virtual LightSample sample_brdf(
          FastRand&,
          const glm::vec3&,
          const glm::vec3&) const;

    private:
      // The reflectance (color) of the specular reflection
      glm::vec3 m_reflectance;
  };

  /**
   * Perfect specular refraction
   *
   * This BTDF oll allways sample EXACTLY one direction given a normal,
   * incident vector and index of refraction. Therefore, pdf = 1.0 and f can
   * just be set to zero as we oll never select a light sample which is exactly
   * in that direction.
   */
  class SpecularRefractionMaterial : public Material
  {
    public:
      SpecularRefractionMaterial(float);

      virtual glm::vec3 brdf(
          const glm::vec3&,
          const glm::vec3&,
          const glm::vec3&) const;

      virtual LightSample sample_brdf(
          FastRand&,
          const glm::vec3&,
          const glm::vec3&) const;

    private:
      // Index of refraction
      float m_ior;

      SpecularReflectionMaterial m_refmat;
  };

  /**
   * Fresnel blending
   *
   * This Material actually combines two brdfs of a view dependent fresnel
   * term. We use the Schlick approximation to the real fresnel equations,
   * which irks quite well for conductors.
   */
  class FresnelBlendMaterial : public Material
  {
    public:
      FresnelBlendMaterial(const Material*, const Material*, float);

      virtual glm::vec3 brdf(
          const glm::vec3&,
          const glm::vec3&,
          const glm::vec3&) const;

      virtual LightSample sample_brdf(
          FastRand&,
          const glm::vec3&,
          const glm::vec3&) const;

    private:
      const Material* m_onReflectionMaterial;
      const Material* m_onRefractionMaterial;
      float m_R0;
  };

  /**
   * Linear blending
   *
   * This Material combines two brdfs linearly as
   *
   *   w * M1 + (1 - w) * M2
   */
  class BlendMaterial : public Material
  {
    public:
      BlendMaterial(const Material*, const Material*, float);

      virtual glm::vec3 brdf(
          const glm::vec3&,
          const glm::vec3&,
          const glm::vec3&) const;

      virtual LightSample sample_brdf(
          FastRand&,
          const glm::vec3&,
          const glm::vec3&) const;

    private:
      const Material* m_firstMaterial;
      const Material* m_secondMaterial;
      float m_w;
  };
}

#endif /* end of include guard: MATERIAL_HPP_FNROXKUG */
