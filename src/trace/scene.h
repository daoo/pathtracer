#ifndef TRACE_SCENE_H_
#define TRACE_SCENE_H_

#include <map>
#include <string>
#include <vector>

#include "geometry/ray.h"
#include "kdtree/array.h"
#include "trace/camera.h"
#include "trace/light.h"
#include "trace/material.h"
#include "wavefront/mtl.h"
#include "wavefront/obj.h"

namespace trace {
std::vector<SphereLight> lights_from_mtl(const wavefront::Mtl& mtl);

std::vector<Camera> cameras_from_mtl(const wavefront::Mtl& mtl);

std::map<std::string, Material*> materials_from_mtl(const wavefront::Mtl& mtl);

std::vector<geometry::Triangle> triangles_from_obj(const wavefront::Obj& obj);

void update_pointer_to_material(
    const std::map<std::string, Material*>& materials,
    std::vector<geometry::Triangle>& triangles);

kdtree::KdTreeArray kdtree_from_triangles(
    const std::vector<geometry::Triangle>& triangles);

struct Scene {
  kdtree::KdTreeArray kdtree;
  std::vector<Camera> cameras;
  std::vector<SphereLight> lights;
  std::vector<geometry::Triangle> triangles;
};

Scene new_scene(const wavefront::Obj& obj, const wavefront::Mtl& mtl);
}  // namespace trace

#endif  // TRACE_SCENE_H_
