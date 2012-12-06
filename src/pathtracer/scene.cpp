#include "scene.hpp"
#include "mcsampling.hpp"

using namespace glm;
using namespace math;
using namespace std;

namespace {
  void buildFromObj(vector<Light>& lights, vector<Camera>& cameras,
      vector<Triangle>& triangles, const OBJModel& model) {
    for (auto kv : model.m_lights) {
      lights.push_back(
          { kv.second.radius
          , kv.second.position
          , kv.second.intensity * kv.second.color
          });
    }

    for (auto kv : model.m_cameras) {
      cameras.push_back(
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

        triangles.push_back(triangle);
      }
    }
  }
}

Scene::Scene(const OBJModel& model) {
  assert(!model.m_cameras.empty());

  vector<Triangle> triangles;
  buildFromObj(m_lights, m_cameras, triangles, model);
  kdtree::buildTree(m_kdtree, triangles);
}

bool Scene::allIntersection(Ray& ray, Intersection& isect) const {
  return kdtree::intersects(m_kdtree, ray, isect);
}

bool Scene::anyIntersection(const Ray& ray) const {
  return kdtree::intersects(m_kdtree, ray);
}
