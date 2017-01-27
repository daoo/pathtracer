#include "geometry/ray.hpp"
#include "trace/camera.hpp"

#include <glm/glm.hpp>
#include <iostream>

using namespace glm;
using namespace std;
using namespace trace;

void print(const Pinhole& pinhole, const vec2& p) {
  Ray ray = pinhole.ray(p.x, p.y);
  vec3 q = ray.param(1.0);

  cout << "(" << p.x << ", " << p.y << ")";
  cout << " = ";
  cout << "(" << q.x << ", " << q.y << ", " << q.z << ")\n";
}

int main(int, char* []) {
  Camera camera({0, 0, 1}, {0, 0, 0}, {0, 1, 0}, 45);
  Pinhole pinhole(camera, 100, 100);

  print(pinhole, {0, 0});
  print(pinhole, {0, 1});
  print(pinhole, {1, 0});
  print(pinhole, {1, 1});

  return 0;
}
