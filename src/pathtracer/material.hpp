#ifndef MATERIAL_HPP_FNROXKUG
#define MATERIAL_HPP_FNROXKUG

#include "intersection.hpp"
#include "mcsampling.hpp"
#include "texture.hpp"
#include "util/fastrand.hpp"

#include <glm/glm.hpp>

/**
 * Abstract base class for materials.
 */
class Material {
  public:
    virtual glm::vec3 f(
        const glm::vec3& wi, const glm::vec3& wo, const Intersection&) const = 0;

    virtual glm::vec3 sample_f(
        FastRand&, const glm::vec3& wi, glm::vec3& wo, const Intersection&, float& pdf) const = 0;
};

/**
 * Diffuse reflection
 *
 * The BRDF for diffuse reflection is constant. It can be importance sampled
 * on the cosine term to improve sampling quality somewhat.
 */
class DiffuseMaterial : public Material {
  public:
    // The reflectance (color) of the material
    glm::vec3 m_reflectance;
    Texture* m_reflectanceMap;

    virtual glm::vec3 f(const glm::vec3&, const glm::vec3&, const Intersection&) const;
    virtual glm::vec3 sample_f(FastRand&, const glm::vec3&, glm::vec3&, const Intersection&, float&) const;
};

/**
 * Perfect specular reflection
 *
 * This BRDF will allways sample EXACTLY one direction given a normal and an
 * incident vector. Therefore, pdf = 1.0 and f can just be set to
 * zero as we will never select a light sample which is exactly in that
 * direction.
 */
class SpecularReflectionMaterial : public Material {
  public:
    // The reflectance (color) of the specular reflection
    glm::vec3 m_reflectance;

    virtual glm::vec3 f(const glm::vec3&, const glm::vec3&, const Intersection&) const;
    virtual glm::vec3 sample_f(FastRand&, const glm::vec3&, glm::vec3&, const Intersection&, float&) const;
};

/**
 * Perfect specular refraction
 *
 * This BTDF oll allways sample EXACTLY one direction given a normal,
 * incident vector and index of refraction. Therefore, pdf = 1.0 and f can
 * just be set to zero as we oll never select a light sample which is exactly
 * in that direction.
 */
class SpecularRefractionMaterial : public Material {
  public:
    // Index of refraction
    float m_ior;

    virtual glm::vec3 f(const glm::vec3&, const glm::vec3&, const Intersection&) const;
    virtual glm::vec3 sample_f(FastRand&, const glm::vec3&, glm::vec3&, const Intersection&, float&) const;
};

/**
 * Fresnel blending
 *
 * This Material actually combines two bxdfs oth a view dependent fresnel
 * term. We use the Schlick approximation to the real fresnel equations,
 * which irks quite well for conductors.
 */
class FresnelBlendMaterial : public Material {
  public:
    Material *m_onReflectionMaterial;
    Material *m_onRefractionMaterial;
    float m_R0;
    float R(const glm::vec3& i, const glm::vec3& n) const;

    virtual glm::vec3 f(const glm::vec3&, const glm::vec3&, const Intersection&) const;
    virtual glm::vec3 sample_f(FastRand&, const glm::vec3&, glm::vec3&, const Intersection&, float&) const;
};

/**
 * Linear blending
 *
 * This Material combines two bxdfs linearly as: w*M1 + (1.0-w)*M2
 */
class BlendMaterial : public Material {
  public:
    Material *m_firstMaterial;
    Material *m_secondMaterial;
    float m_w;

    virtual glm::vec3 f(const glm::vec3&, const glm::vec3&, const Intersection&) const;
    virtual glm::vec3 sample_f(FastRand&, const glm::vec3&, glm::vec3&, const Intersection&, float&) const;
};

#endif /* end of include guard: MATERIAL_HPP_FNROXKUG */
