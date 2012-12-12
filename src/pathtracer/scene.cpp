#include "scene.hpp"
#include "mcsampling.hpp"

using namespace glm;
using namespace math;
using namespace std;

namespace {
  void buildFromObj(const OBJModel& model,
      vector<SphereLight>& lights, vector<Camera>& cameras,
      vector<Material*>& materials, vector<Texture*> textures,
      vector<Triangle>& triangles) {
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

      Texture* reflectanceMap = nullptr;
      if (!chunk.material->diffuseReflectanceMap.empty()) {
        reflectanceMap = new Texture;
        textureLoad(*reflectanceMap, chunk.material->diffuseReflectanceMap);
      }
      DiffuseMaterial* diffuse =
        new DiffuseMaterial(chunk.material->diffuseReflectance, reflectanceMap);

      SpecularReflectionMaterial* specularReflection =
        new SpecularReflectionMaterial(chunk.material->specularReflectance);

      SpecularRefractionMaterial* specularRefraction =
        new SpecularRefractionMaterial(chunk.material->indexOfRefraction);

      BlendMaterial* blend1 =
        new BlendMaterial(specularRefraction, diffuse, chunk.material->transparency);

      FresnelBlendMaterial* fresnel =
        new FresnelBlendMaterial(specularReflection, blend1, chunk.material->reflAt0Deg);

      BlendMaterial* blend0 =
        new BlendMaterial(fresnel, blend1, chunk.material->reflAt90Deg);

      textures.push_back(reflectanceMap);
      materials.push_back(diffuse);
      materials.push_back(specularReflection);
      materials.push_back(specularRefraction);
      materials.push_back(fresnel);
      materials.push_back(blend0);
      materials.push_back(blend1);

      for (size_t j = 0; j < chunk.m_positions.size(); j += 3) {
        Triangle triangle;

        triangle.v0  = chunk.m_positions[j + 0];
        triangle.v1  = chunk.m_positions[j + 1];
        triangle.v2  = chunk.m_positions[j + 2];
        triangle.n0  = chunk.m_normals[j + 0];
        triangle.n1  = chunk.m_normals[j + 1];
        triangle.n2  = chunk.m_normals[j + 2];
        triangle.uv0 = chunk.m_uvs[j + 0];
        triangle.uv1 = chunk.m_uvs[j + 1];
        triangle.uv2 = chunk.m_uvs[j + 2];

        triangle.m_material = blend0;

        triangles.push_back(triangle);
      }
    }
  }
}

Scene::Scene(const OBJModel& model) {
  assert(!model.m_cameras.empty());

  buildFromObj(model, m_lights, m_cameras, m_material, m_textures, m_triangles);
  kdtree::buildTree(m_kdtree, m_triangles);
}

Scene::~Scene() { }

bool Scene::allIntersection(Ray& ray, Intersection& isect) const {
  return kdtree::intersects(m_kdtree, ray, isect);
}

bool Scene::anyIntersection(const Ray& ray) const {
  return kdtree::intersects(m_kdtree, ray);
}
