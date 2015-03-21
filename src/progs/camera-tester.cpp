#include "trace/camera.hpp"
#include "trace/geometry/ray.hpp"
#include "trace/pathtracer.hpp"

#include <glm/glm.hpp>
#include <iostream>

using namespace glm;
using namespace std;
using namespace trace;

void print(Pinhole pinhole, vec2 p)
{
  Ray ray = pinhole_ray(pinhole, p.x, p.y);
  vec3 q = ray.param(1.0);

  cout << "(" << p.x << ", " << p.y << ")";
  cout << " = ";
  cout << "(" << q.x << ", " << q.y << ", " << q.z << ")\n";
}

int main(int argc, char* argv[])
{
  Camera camera   = new_camera({0, 0, 1}, {0, 0, 0}, {0, 1, 0}, 45);
  Pinhole pinhole = new_pinhole(camera, 100, 100);

  print(pinhole, {0, 0});
  print(pinhole, {0, 1});
  print(pinhole, {1, 0});
  print(pinhole, {1, 1});

  return 0;
}
