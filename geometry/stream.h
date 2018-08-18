#ifndef GEOMETRY_STREAM_H_
#define GEOMETRY_STREAM_H_

#include <glm/glm.hpp>

#include <iostream>

#include "geometry/aabb.h"
#include "geometry/triangle.h"

std::ostream& operator<<(std::ostream& stream, const glm::vec2& v) {
  stream << '{' << v.x << ',' << v.y << '}';
  return stream;
}

std::ostream& operator<<(std::ostream& stream, const glm::vec3& v) {
  stream << '{' << v.x << ',' << v.y << ',' << v.z << '}';
  return stream;
}

std::ostream& operator<<(std::ostream& stream, const geometry::Aabb& box) {
  stream << "Aabb{" << box.GetCenter() << ',' << box.GetHalf() << '}';
  return stream;
}

std::ostream& operator<<(std::ostream& stream, const geometry::Triangle& tri) {
  stream << "Triangle{";
  stream << tri.v0 << "," << tri.v1 << "," << tri.v2 << ",";
  stream << tri.n0 << "," << tri.n1 << "," << tri.n2 << ",";
  stream << tri.uv0 << "," << tri.uv1 << "," << tri.uv2 << "}";
  return stream;
}

#endif  // GEOMETRY_STREAM_H_
