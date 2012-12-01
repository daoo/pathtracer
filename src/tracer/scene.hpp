#ifndef SCENE_HPP_BOFJZX4D
#define SCENE_HPP_BOFJZX4D

#include "math/ray.hpp"
#include "tracer/camera.hpp"
#include "tracer/light.hpp"
#include "tracer/material.hpp"
#include "util/objmodel.hpp"

class Triangle {
  public:
    glm::vec3 v0, v1, v2;
    glm::vec3 n0, n1, n2;
    glm::vec2 uv0, uv1, uv2;
    Material* m_material;
};

class Scene {
  public:
    std::vector<Light> m_lights;
    std::vector<Camera> m_cameras;
    std::vector<Triangle> m_triangles;

    void buildFromObj(OBJModel* model);
    bool allIntersection(math::Ray& r, Intersection& isect) const;
    bool anyIntersection(const math::Ray& r) const;
};

#endif /* end of include guard: SCENE_HPP_BOFJZX4D */
