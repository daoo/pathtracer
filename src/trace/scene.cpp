#include "trace/scene.hpp"

#include "trace/mcsampling.hpp"
#include <glm/gtc/epsilon.hpp>
#include <map>

using namespace glm;
using namespace math;
using namespace std;
namespace trace
{
  namespace
  {
    constexpr float EPSILON = 0.0001f;

    vec2 to_glm(const obj::Vec2& v) { return vec2(v.x, v.y); }
    vec3 to_glm(const obj::Vec3& v) { return vec3(v.x, v.y, v.z); }

    void buildFromObj(const obj::Obj& obj, const obj::Mtl& mtl,
        vector<SphereLight>& lights, vector<Camera>& cameras,
        vector<Triangle>& triangles)
    {
      for (const obj::Light& light : mtl.lights) {
        lights.push_back(
            { light.radius
            , to_glm(light.position)
            , light.intensity * to_glm(light.color)
            });
      }

      for (const obj::Camera& camera : mtl.cameras) {
        cameras.push_back(
            { to_glm(camera.position)
            , normalize(to_glm(camera.target) - to_glm(camera.position))
            , normalize(to_glm(camera.up))
            , camera.fov
            });
      }

      map<string, Material*> materials;
      for (const obj::Material& mat : mtl.materials) {
        DiffuseMaterial* diffuse =
          new DiffuseMaterial(to_glm(mat.diffuse));

        SpecularRefractionMaterial* specularRefraction =
          new SpecularRefractionMaterial(mat.ior);

        Material* blend1 = nullptr;
        if (epsilonEqual(mat.transparency, 1.0f, EPSILON)) {
          blend1 = specularRefraction;
          delete diffuse;
        } else if (epsilonEqual(mat.transparency, 0.0f, EPSILON)) {
          blend1 = diffuse;
          delete specularRefraction;
        } else {
          blend1 = new BlendMaterial(
              specularRefraction, diffuse, mat.transparency);
        }

        SpecularReflectionMaterial* specularReflection =
          new SpecularReflectionMaterial(to_glm(mat.specular));

        FresnelBlendMaterial* fresnel =
          new FresnelBlendMaterial(specularReflection, blend1, mat.reflAt0Deg);

        Material* blend0 = nullptr;
        if (epsilonEqual(mat.reflAt90Deg, 1.0f, EPSILON)) {
          blend0 = fresnel;
        } else if (epsilonEqual(mat.reflAt90Deg, 0.0f, EPSILON)) {
          blend0 = blend1;
          delete fresnel;
        } else {
          blend0 = new BlendMaterial(fresnel, blend1, mat.reflAt90Deg);
        }

        materials[mat.name] = blend0;
      }

      for (const obj::Chunk& chunk : obj.chunks) {
        Material* mat = materials[chunk.material];
        for (const obj::Triangle& tri : chunk.triangles) {
          triangles.push_back(
              { to_glm(tri.v1.p)
              , to_glm(tri.v2.p)
              , to_glm(tri.v3.p)
              , to_glm(tri.v1.n)
              , to_glm(tri.v2.n)
              , to_glm(tri.v3.n)
              , to_glm(tri.v1.t)
              , to_glm(tri.v2.t)
              , to_glm(tri.v3.t)
              , mat
              });
        }
      }
    }
  }

  Scene::Scene() { }

  Scene::Scene(const obj::Obj& obj, const obj::Mtl& mtl)
  {
    buildFromObj(obj, mtl, m_lights, m_cameras, m_triangles);
    kdtree::buildTree(m_kdtree, m_triangles);
  }

  Scene::~Scene() { }
}
