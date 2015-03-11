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

    void buildFromObj(const wavefront::Obj& obj, const wavefront::Mtl& mtl,
        vector<SphereLight>& lights, vector<Camera>& cameras,
        vector<Triangle>& triangles)
    {
      for (const wavefront::Light& light : mtl.lights) {
        lights.push_back(
            { light.radius
            , light.center
            , light.intensity * light.color
            });
      }

      for (const wavefront::Camera& camera : mtl.cameras) {
        cameras.push_back(
            { camera.position
            , normalize(camera.target - camera.position)
            , normalize(camera.up)
            , camera.fov
            });
      }

      map<string, Material*> materials;
      for (const wavefront::Material& mat : mtl.materials) {
        DiffuseMaterial* diffuse =
          new DiffuseMaterial(mat.diffuse);

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
          new SpecularReflectionMaterial(mat.specular);

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

      for (const wavefront::Chunk& chunk : obj.chunks) {
        Material* mat = materials[chunk.material];
        for (const wavefront::Face& polygon : chunk.polygons) {
          triangles.push_back(
              { indexVertex(obj, polygon.p1.v)
              , indexVertex(obj, polygon.p2.v)
              , indexVertex(obj, polygon.p3.v)
              , indexNormal(obj, polygon.p1.n)
              , indexNormal(obj, polygon.p2.n)
              , indexNormal(obj, polygon.p3.n)
              , indexTexCoord(obj, polygon.p1.t)
              , indexTexCoord(obj, polygon.p2.t)
              , indexTexCoord(obj, polygon.p3.t)
              , mat
              });
        }
      }
    }
  }

  Scene::Scene() { }

  Scene::Scene(const wavefront::Obj& obj, const wavefront::Mtl& mtl)
  {
    buildFromObj(obj, mtl, m_lights, m_cameras, m_triangles);
    kdtree::buildTree(m_kdtree, m_triangles);
  }

  Scene::~Scene() { }
}
