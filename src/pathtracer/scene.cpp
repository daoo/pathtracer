#include "scene.hpp"
#include "mcsampling.hpp"

#include <map>

using namespace glm;
using namespace math;
using namespace std;

namespace
{
  vec2 to_glm(const objloader::Vec2& v) { return vec2(v.x, v.y); }
  vec3 to_glm(const objloader::Vec3& v) { return vec3(v.x, v.y, v.z); }

  void buildFromObj(const objloader::Obj& obj, const objloader::Mtl& mtl,
      vector<SphereLight>& lights, vector<Camera>& cameras,
      vector<Triangle>& triangles)
  {
    for (const objloader::Light& light : mtl.lights) {
      lights.push_back(
          { light.radius
          , to_glm(light.position)
          , light.intensity * to_glm(light.color)
          });
    }

    for (const objloader::Camera& camera : mtl.cameras) {
      cameras.push_back(
          { to_glm(camera.position)
          , normalize(to_glm(camera.target) - to_glm(camera.position))
          , normalize(to_glm(camera.up))
          , camera.fov
          });
    }

    map<string, Material*> materials;
    for (const objloader::Material& mat : mtl.materials) {
      Texture* reflectanceMap = nullptr;
      if (!mat.diffuseMap.empty()) {
        reflectanceMap = new Texture;
        textureLoad(*reflectanceMap, mat.diffuseMap);
      }

      DiffuseMaterial* diffuse =
        new DiffuseMaterial(to_glm(mat.diffuse), reflectanceMap);

      SpecularReflectionMaterial* specularReflection =
        new SpecularReflectionMaterial(to_glm(mat.specular));

      SpecularRefractionMaterial* specularRefraction =
        new SpecularRefractionMaterial(mat.ior);

      BlendMaterial* blend1 =
        new BlendMaterial(specularRefraction, diffuse, mat.transparency);

      FresnelBlendMaterial* fresnel =
        new FresnelBlendMaterial(specularReflection, blend1, mat.reflAt0Deg);

      BlendMaterial* blend0 =
        new BlendMaterial(fresnel, blend1, mat.reflAt90Deg);

      materials[mat.name] = blend0;
    }

    for (const objloader::Chunk& chunk : obj.chunks) {
      Material* mat = materials[chunk.material];
      for (const objloader::Triangle& tri : chunk.triangles) {
        triangles.push_back(
            { to_glm(obj.vertices[tri.p1.v])
            , to_glm(obj.vertices[tri.p2.v])
            , to_glm(obj.vertices[tri.p3.v])
            , to_glm(obj.normals[tri.p1.n])
            , to_glm(obj.normals[tri.p2.n])
            , to_glm(obj.normals[tri.p3.n])
            , to_glm(obj.texcoords[tri.p1.t])
            , to_glm(obj.texcoords[tri.p2.t])
            , to_glm(obj.texcoords[tri.p3.t])
            , mat
            });
      }
    }
  }
}

Scene::Scene() { }

Scene::Scene(const objloader::Obj& obj, const objloader::Mtl& mtl)
{
  buildFromObj(obj, mtl, m_lights, m_cameras, m_triangles);
  kdtree::buildTree(m_kdtree, m_triangles);
}

Scene::~Scene() { }

bool Scene::allIntersection(Ray& ray, Intersection& isect) const
{
  return kdtree::intersects(m_kdtree, ray, isect);
}

bool Scene::anyIntersection(const Ray& ray) const
{
  return kdtree::intersects(m_kdtree, ray);
}
