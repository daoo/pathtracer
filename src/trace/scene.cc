#include "trace/scene.h"

#include <glm/gtc/epsilon.hpp>
#include <map>

#include "kdtree/array.h"
#include "kdtree/linked.h"
#include "kdtree/optimize.h"
#include "kdtree/surface_area_heuristic.h"
#include "trace/mcsampling.h"

using glm::vec3;
using std::map;
using std::string;
using std::vector;

namespace trace {
namespace {
constexpr float EPSILON = 0.0001f;

Material* blend1_from_wavefront(const wavefront::Material& material) {
  if (glm::epsilonEqual(material.transparency, 1.0f, EPSILON)) {
    return new SpecularRefractionMaterial(material.ior);
  } else if (glm::epsilonEqual(material.transparency, 0.0f, EPSILON)) {
    return new DiffuseMaterial(material.diffuse);
  } else {
    return new BlendMaterial(new SpecularRefractionMaterial(material.ior),
                             new DiffuseMaterial(material.diffuse),
                             material.transparency);
  }
}

Material* blend0_from_wavefront(const wavefront::Material& material,
                                Material* blend1) {
  if (glm::epsilonEqual(material.refl90, 1.0f, EPSILON)) {
    return new FresnelBlendMaterial(
        new SpecularReflectionMaterial(material.specular), blend1,
        material.refl0);
  } else if (glm::epsilonEqual(material.refl90, 0.0f, EPSILON)) {
    return blend1;
  } else {
    return new BlendMaterial(
        new FresnelBlendMaterial(
            new SpecularReflectionMaterial(material.specular), blend1,
            material.refl0),
        blend1, material.refl90);
  }
}

Material* material_from_wavefront(const wavefront::Material& material) {
  return blend0_from_wavefront(material, blend1_from_wavefront(material));
}
}  // namespace

vector<SphereLight> lights_from_mtl(const wavefront::Mtl& mtl) {
  vector<SphereLight> lights;
  for (const wavefront::Light& light : mtl.lights) {
    lights.push_back(
        {light.center, light.color, light.intensity, light.radius});
  }
  return lights;
}

vector<Camera> cameras_from_mtl(const wavefront::Mtl& mtl) {
  vector<Camera> cameras;
  for (const wavefront::Camera& camera : mtl.cameras) {
    cameras.push_back(Camera(camera.position, camera.target, camera.up,
                             glm::radians(camera.fov)));
  }
  return cameras;
}

map<string, Material*> materials_from_mtl(const wavefront::Mtl& mtl) {
  map<string, Material*> materials;
  for (const wavefront::Material& material : mtl.materials) {
    materials[material.name] = material_from_wavefront(material);
  }

  return materials;
}

vector<geometry::Triangle> triangles_from_obj(const wavefront::Obj& obj) {
  vector<geometry::Triangle> triangles;
  for (const wavefront::Chunk& chunk : obj.chunks) {
    for (const wavefront::Face& polygon : chunk.polygons) {
      // TODO: ensure the lifetime of somehow &chunk.material
      triangles.push_back(
          {index_vertex(obj, polygon.p1.v), index_vertex(obj, polygon.p2.v),
           index_vertex(obj, polygon.p3.v), index_normal(obj, polygon.p1.n),
           index_normal(obj, polygon.p2.n), index_normal(obj, polygon.p3.n),
           index_texcoord(obj, polygon.p1.t), index_texcoord(obj, polygon.p2.t),
           index_texcoord(obj, polygon.p3.t), &chunk.material});
    }
  }
  return triangles;
}

void update_pointer_to_material(
    const std::map<std::string, Material*>& materials,
    std::vector<geometry::Triangle>& triangles) {
  for (geometry::Triangle& triangle : triangles) {
    triangle.tag = materials.at(*static_cast<const std::string*>(triangle.tag));
  }
}

kdtree::KdTreeArray kdtree_from_triangles(
    const vector<geometry::Triangle>& triangles) {
  kdtree::KdNodeLinked* linked = kdtree::build_tree_sah(triangles);
  kdtree::KdTreeArray array = kdtree::optimize(linked);
  delete linked;
  return array;
}

Scene new_scene(const wavefront::Obj& obj, const wavefront::Mtl& mtl) {
  vector<Camera> cameras = cameras_from_mtl(mtl);
  vector<SphereLight> lights = lights_from_mtl(mtl);

  vector<geometry::Triangle> triangles = triangles_from_obj(obj);
  map<string, Material*> materials = materials_from_mtl(mtl);
  update_pointer_to_material(materials, triangles);

  kdtree::KdTreeArray kdtree = kdtree_from_triangles(triangles);

  return {kdtree, cameras, lights, triangles};
}
}  // namespace trace
