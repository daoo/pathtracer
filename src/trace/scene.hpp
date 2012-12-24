#ifndef SCENE_HPP_BOFJZX4D
#define SCENE_HPP_BOFJZX4D

#include "math/ray.hpp"
#include "trace/camera.hpp"
#include "trace/kdtree/tree.hpp"
#include "trace/light.hpp"
#include "trace/material.hpp"
#include "trace/obj/loader.hpp"

namespace trace
{
  class Scene
  {
    public:
      Scene();
      Scene(const obj::Obj&, const obj::Mtl&);
      ~Scene();

      bool allIntersection(math::Ray& r, Intersection& isect) const;
      bool anyIntersection(const math::Ray& r) const;

      const std::vector<Camera>& cameras() const
      {
        return m_cameras;
      }

      const std::vector<SphereLight>& lights() const
      {
        return m_lights;
      }

      const kdtree::KdTree& kdtree() const
      {
        return m_kdtree;
      }

    private:
      std::vector<SphereLight> m_lights;
      std::vector<Camera> m_cameras;
      std::vector<Triangle> m_triangles;

      kdtree::KdTree m_kdtree;

      Scene(const Scene&);
      Scene& operator=(const Scene&);
  };
}

#endif /* end of include guard: SCENE_HPP_BOFJZX4D */
