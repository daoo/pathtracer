#ifndef SCENE_HPP_BOFJZX4D
#define SCENE_HPP_BOFJZX4D

#include "trace/camera.hpp"
#include "trace/geometry/ray.hpp"
#include "trace/kdtree/tree.hpp"
#include "trace/light.hpp"
#include "trace/material.hpp"
#include "trace/wavefront/obj.hpp"
#include "trace/wavefront/mtl.hpp"

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
      const std::map<std::string, Material*> materials);

  kdtree::KdTree kdtree_from_triangles(
      const std::vector<Triangle>& triangles);

  struct Scene
  {
    kdtree::KdTree kdtree;
    std::vector<Camera> cameras;
    std::vector<SphereLight> lights;
    std::vector<Triangle> triangles;
  };

  Scene new_scene(
      const wavefront::Obj& obj,
      const wavefront::Mtl& mtl);
}

#endif /* end of include guard: SCENE_HPP_BOFJZX4D */
