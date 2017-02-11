#ifndef GEOMETRY_TRIRAY_H_
#define GEOMETRY_TRIRAY_H_

#include <experimental/optional>
#include <glm/glm.hpp>
#include <vector>

#include "geometry/ray.h"
#include "geometry/triangle.h"

namespace geometry {
struct TriRayIntersection {
  const Triangle* triangle;
  const Ray* ray;
  float t, u, v;

  inline glm::vec3 get_position() const { return ray->param(t); }

  inline glm::vec3 get_normal() const {
    return glm::normalize((1.0f - (u + v)) * triangle->n0 + u * triangle->n1 +
                          v * triangle->n2);
  }
};

std::experimental::optional<TriRayIntersection> intersect(const Triangle& tri,
                                                          const Ray& ray);

bool intersect_triangles(const std::vector<Triangle>& triangles,
                         const Ray& ray,
                         float mint,
                         float& maxt,
                         glm::vec3& normal,
                         const void*& tag);
}  // namespace geometry

#endif  // GEOMETRY_TRIRAY_H_
