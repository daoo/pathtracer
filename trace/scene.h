#ifndef TRACE_SCENE_H_
#define TRACE_SCENE_H_

#include <map>
#include <string>
#include <vector>

#include "geometry/triray.h"
#include "kdtree/kdtree.h"
#include "trace/light.h"

namespace geometry {
struct Ray;
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

class Scene {
 public:
  Scene(const wavefront::Obj& obj, const wavefront::Mtl& mtl);

  inline bool AnyIntersect(const geometry::Ray& ray,
                           float tmin,
                           float tmax) const {
    return static_cast<bool>(Intersect(ray, tmin, tmax));
  }

  inline std::optional<geometry::TriRayIntersection>
  Intersect(const geometry::Ray& ray, float tmin, float tmax) const {
    return kdtree::search_tree(kdtree_, ray, tmin, tmax);
  }

  const std::vector<SphereLight>& GetLights() const { return lights_; }

  const std::vector<Camera>& GetCameras() const { return cameras_; }

 private:
  std::vector<geometry::Triangle> triangles_;
  std::map<std::string, Material*> materials_;
  std::vector<Camera> cameras_;
  std::vector<SphereLight> lights_;
  kdtree::KdTree kdtree_;
};
}  // namespace trace

#endif  // TRACE_SCENE_H_
