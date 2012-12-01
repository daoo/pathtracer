#include "scene.hpp"
#include "mcsampling.hpp"

#include <iostream>

using namespace glm;
using namespace math;
using namespace std;

namespace {
  bool intersects(const Triangle& tri, const Ray& r) {
    constexpr float epsilon = 0.00001f;

    const vec3 d  = r.d;
    const vec3 o  = r.o;
    const vec3 e1 = tri.v1 - tri.v0;
    const vec3 e2 = tri.v2 - tri.v0;
    const vec3 q  = cross(d, e2);

    float a = dot(e1, q);
    if (a > -epsilon && a < epsilon)
      return false;

    float f = 1.0f/a;
    vec3 s  = o - tri.v0;
    float u = f * dot(s, q);
    if (u < 0.0 || u > 1.0)
      return false;

    vec3 R  = cross(s, e1);
    float v = f * dot(d, R);
    if (v < 0.0 || u + v > 1.0)
      return false;

    float t = f * dot(e2, R);
    if (t < r.mint || t > r.maxt)
      return false;

    return true;
  }

  bool findIntersection(const Triangle& tri, Ray& r, Intersection& i) {
    constexpr float epsilon = 0.00001f;

    vec3 d  = r.d;
    vec3 o  = r.o;
    vec3 e1 = tri.v1 - tri.v0;
    vec3 e2 = tri.v2 - tri.v0;
    vec3 q  = cross(d, e2);

    float a = dot(e1, q);
    if (a > -epsilon && a < epsilon)
      return false;

    float f = 1.0f/a;
    vec3 s  = o - tri.v0;
    float u = f * dot(s, q);
    if (u < 0.0 || u > 1.0)
      return false;

    vec3 R  = cross(s, e1);
    float v = f * dot(d, R);
    if (v < 0.0 || u+v > 1.0)
      return false;

    float t = f * dot(e2, R);
    if (t < r.mint || t > r.maxt)
      return false;

    r.maxt       = t;
    i.m_position = r(t);
    i.m_normal   = normalize((1.0f - (u + v)) * tri.n0 + u * tri.n1 + v * tri.n2);
    i.m_material = tri.m_material;
    return true;
  }
}

// -----------------------------------------------------------------------
// Find the first intersection between a ray and the scene.
bool Scene::allIntersection(Ray& r, Intersection& isect) const {
  bool foundIntersection = false;
  for (size_t i = 0; i < m_triangles.size(); ++i) {
    foundIntersection |= findIntersection(m_triangles[i], r, isect);
  }
  return foundIntersection;
}

// -----------------------------------------------------------------------
// Return wether there there is ANY intersection between the ray and the scene
bool Scene::anyIntersection(const Ray& r) const {
  for (size_t i = 0; i < m_triangles.size(); ++i) {
    if (intersects(m_triangles[i], r))
      return true;
  }
  return false;
}

// -----------------------------------------------------------------------
// Buid a Scene object from an OBJ file
void Scene::buildFromObj(const OBJModel& model) {
  for (auto kv : model.m_lights) {
    m_lights.push_back(
        { kv.second.radius
        , kv.second.position
        , kv.second.intensity * kv.second.color
        });
  }

  for (auto kv : model.m_cameras) {
    m_cameras.push_back(
        { kv.second.position
        , normalize(kv.second.target - kv.second.position)
        , normalize(kv.second.up)
        , kv.second.fov
        });
  }

  for (size_t i = 0; i < model.m_chunks.size(); ++i) {
    const OBJModel::Chunk& chunk = model.m_chunks[i];
    Material* material;

    DiffuseMaterial* diffuse = new DiffuseMaterial;
    diffuse->m_reflectance   = chunk.material->diffuseReflectance;
    if (!chunk.material->diffuseReflectanceMap.empty()) {
      diffuse->m_reflectanceMap = new Texture;
      diffuse->m_reflectanceMap->load(chunk.material->diffuseReflectanceMap);
    } else {
      diffuse->m_reflectanceMap = nullptr;
    }

    SpecularReflectionMaterial* specularReflection = new SpecularReflectionMaterial;
    specularReflection->m_reflectance              = chunk.material->specularReflectance;

    SpecularRefractionMaterial* specularRefraction = new SpecularRefractionMaterial;
    specularRefraction->m_ior                      = chunk.material->indexOfRefraction;

    BlendMaterial* blend1    = new BlendMaterial;
    blend1->m_w              = chunk.material->transparency;
    blend1->m_firstMaterial  = specularRefraction;
    blend1->m_secondMaterial = diffuse;

    FresnelBlendMaterial* fresnel   = new FresnelBlendMaterial;
    fresnel->m_R0                   = chunk.material->reflAt0Deg;
    fresnel->m_onReflectionMaterial = specularReflection;
    fresnel->m_onRefractionMaterial = blend1;

    BlendMaterial* blend0    = new BlendMaterial;
    blend0->m_w              = chunk.material->reflAt90Deg;
    blend0->m_firstMaterial  = fresnel;
    blend0->m_secondMaterial = blend1;

    material = blend0;

    for (size_t j = 0; j < chunk.m_positions.size() / 3; ++j) {
      Triangle triangle;

      triangle.v0  = chunk.m_positions[j*3 + 0];
      triangle.v1  = chunk.m_positions[j*3 + 1];
      triangle.v2  = chunk.m_positions[j*3 + 2];
      triangle.n0  = chunk.m_normals[j*3 + 0];
      triangle.n1  = chunk.m_normals[j*3 + 1];
      triangle.n2  = chunk.m_normals[j*3 + 2];
      triangle.uv0 = chunk.m_uvs[j*3+0];
      triangle.uv1 = chunk.m_uvs[j*3+1];
      triangle.uv2 = chunk.m_uvs[j*3+2];

      triangle.m_material = material;

      m_triangles.push_back(triangle);
    }
  }
}
