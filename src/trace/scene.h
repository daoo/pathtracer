#ifndef TRACE_SCENE_H_
#define TRACE_SCENE_H_

#include <map>
#include <string>
#include <vector>

#include "kdtree/linked.h"
#include "trace/light.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

namespace wavefront {
struct Mtl;
struct Obj;
}  // namespace wavefront

namespace trace {
struct Camera;
class Material;

std::vector<SphereLight> lights_from_mtl(const wavefront::Mtl& mtl);

std::vector<Camera> cameras_from_mtl(const wavefront::Mtl& mtl);

std::map<std::string, Material*> materials_from_mtl(const wavefront::Mtl& mtl);

std::vector<geometry::Triangle> triangles_from_obj(const wavefront::Obj& obj);

struct Scene {
  Scene(const wavefront::Obj& obj, const wavefront::Mtl& mtl);

  std::vector<geometry::Triangle> triangles;
  std::map<std::string, Material*> materials;
  std::vector<Camera> cameras;
  std::vector<SphereLight> lights;
  kdtree::KdTreeLinked kdtree;
};
}  // namespace trace

#endif  // TRACE_SCENE_H_
