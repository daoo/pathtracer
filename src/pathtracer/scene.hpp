#ifndef SCENE_HPP_BOFJZX4D
#define SCENE_HPP_BOFJZX4D

#include "camera.hpp"
#include "kdtree/dt/linked.hpp"
#include "light.hpp"
#include "material.hpp"
#include "math/ray.hpp"
#include "util/objmodel.hpp"

class Scene {
  public:
    std::vector<Light> m_lights;
    std::vector<Camera> m_cameras;
    kdtree::KdTreeLinked m_kdtree;

    void buildFromObj(const OBJModel& model);
    bool allIntersection(math::Ray& r, Intersection& isect) const;
    bool anyIntersection(const math::Ray& r) const;
};

#endif /* end of include guard: SCENE_HPP_BOFJZX4D */
