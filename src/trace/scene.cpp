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

    void build_from_obj(
        const wavefront::Obj& obj,
        const wavefront::Mtl& mtl,
        vector<SphereLight>& lights,
        vector<Camera>& cameras,
        vector<Triangle>& triangles)
    {
      for (const wavefront::Light& light : mtl.lights) {
        lights.push_back(new_light(
              light.center,
              light.color,
              light.intensity,
              light.radius));
      }

      for (const wavefront::Camera& camera : mtl.cameras) {
        cameras.push_back(new_camera(
              camera.position,
              camera.target,
              camera.up,
              camera.fov));
      }

      map<string, Material*> materials;
      for (const wavefront::Material& mat : mtl.materials) {
        DiffuseMaterial* diffuse =
          new DiffuseMaterial(mat.diffuse);

        SpecularRefractionMaterial* specular_refraction =
          new SpecularRefractionMaterial(mat.ior);

        Material* blend1 = nullptr;
        if (epsilonEqual(mat.transparency, 1.0f, EPSILON)) {
          blend1 = specular_refraction;
          delete diffuse;
        } else if (epsilonEqual(mat.transparency, 0.0f, EPSILON)) {
          blend1 = diffuse;
          delete specular_refraction;
        } else {
          blend1 = new BlendMaterial(
              specular_refraction, diffuse, mat.transparency);
        }

        SpecularReflectionMaterial* specular_reflection =
          new SpecularReflectionMaterial(mat.specular);

        FresnelBlendMaterial* fresnel =
          new FresnelBlendMaterial(specular_reflection, blend1, mat.refl0);

        Material* blend0 = nullptr;
        if (epsilonEqual(mat.refl90, 1.0f, EPSILON)) {
          blend0 = fresnel;
        } else if (epsilonEqual(mat.refl90, 0.0f, EPSILON)) {
          blend0 = blend1;
          delete fresnel;
        } else {
          blend0 = new BlendMaterial(fresnel, blend1, mat.refl90);
        }

        materials[mat.name] = blend0;
      }

      for (const wavefront::Chunk& chunk : obj.chunks) {
        Material* mat = materials[chunk.material];
        for (const wavefront::Face& polygon : chunk.polygons) {
          triangles.push_back(
              { index_vertex(obj, polygon.p1.v)
              , index_vertex(obj, polygon.p2.v)
              , index_vertex(obj, polygon.p3.v)
              , index_normal(obj, polygon.p1.n)
              , index_normal(obj, polygon.p2.n)
              , index_normal(obj, polygon.p3.n)
              , index_texcoord(obj, polygon.p1.t)
              , index_texcoord(obj, polygon.p2.t)
              , index_texcoord(obj, polygon.p3.t)
              , mat
              });
        }
      }
    }
  }

  Scene::Scene() { }

  Scene::Scene(const wavefront::Obj& obj, const wavefront::Mtl& mtl)
  {
    build_from_obj(obj, mtl, m_lights, m_cameras, m_triangles);
    kdtree::build_tree(m_kdtree, m_triangles);
  }

  Scene::~Scene() { }
}
