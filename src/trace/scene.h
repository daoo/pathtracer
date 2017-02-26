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

struct Scene {
  Scene(const wavefront::Obj& obj, const wavefront::Mtl& mtl);

  std::vector<Camera> cameras;
  std::vector<SphereLight> lights;
  kdtree::KdTreeArray kdtree;
};
}  // namespace trace

#endif  // TRACE_SCENE_H_
