#ifndef GEOMETRY_TRIRAY_H_
#define GEOMETRY_TRIRAY_H_

#include <glm/glm.hpp>
#include <vector>

namespace geometry {
struct Ray;
struct Triangle;

bool triray(const Triangle& tri, const Ray& ray, float& t, glm::vec3& n);

bool intersect_triangles(const std::vector<Triangle>& triangles,
                         const Ray& ray,
                         float mint,
                         float& maxt,
                         glm::vec3& normal,
                         const void*& tag);
}  // namespace geometry

#endif  // GEOMETRY_TRIRAY_H_
