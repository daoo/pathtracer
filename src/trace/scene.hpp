#ifndef SCENE_HPP_BOFJZX4D
#define SCENE_HPP_BOFJZX4D

#include "geometry/ray.hpp"
#include "kdtree/array.hpp"
#include "trace/camera.hpp"
#include "trace/light.hpp"
#include "trace/material.hpp"
#include "wavefront/mtl.hpp"
#include "wavefront/obj.hpp"

#include <map>
#include <string>
#include <vector>

namespace trace
{
  std::vector<SphereLight> lights_from_mtl(
      const wavefront::Mtl& mtl);

  std::vector<Camera> cameras_from_mtl(
      const wavefront::Mtl& mtl);

  std::map<std::string, Material*> materials_from_mtl(
      const wavefront::Mtl& mtl);

  std::vector<Triangle> triangles_from_obj(
      const wavefront::Obj& obj,
      const std::map<std::string, Material*>& materials);

  kdtree::KdTreeArray kdtree_from_triangles(
      const std::vector<Triangle>& triangles);

  struct Scene
  {
    kdtree::KdTreeArray kdtree;
    std::vector<Camera> cameras;
    std::vector<SphereLight> lights;
    std::vector<Triangle> triangles;
  };

  Scene new_scene(
      const wavefront::Obj& obj,
      const wavefront::Mtl& mtl);
}

#endif /* end of include guard: SCENE_HPP_BOFJZX4D */
