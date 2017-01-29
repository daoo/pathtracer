#include <glm/glm.hpp>
#include <iostream>

#include "geometry/ray.h"
#include "trace/camera.h"

void print(const trace::Pinhole& pinhole, const glm::vec2& p) {
  geometry::Ray ray = pinhole.ray(p.x, p.y);
  glm::vec3 q = ray.param(1.0);

  std::cout << "(" << p.x << ", " << p.y << ")";
  std::cout << " = ";
  std::cout << "(" << q.x << ", " << q.y << ", " << q.z << ")\n";
}

int main(int, char* []) {
  trace::Camera camera({0, 0, 1}, {0, 0, 0}, {0, 1, 0}, 45);
  trace::Pinhole pinhole(camera, 100, 100);

  print(pinhole, {0, 0});
  print(pinhole, {0, 1});
  print(pinhole, {1, 0});
  print(pinhole, {1, 1});

  return 0;
}
