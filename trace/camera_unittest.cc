#include <glm/glm.hpp>

#include "geometry/ray.h"
#include "tests/catch.h"
#include "trace/camera.h"

TEST_CASE("pinhole gives correct ray", "[camera]") {
  trace::Camera camera({0, 0, -1}, {0, 0, 0}, {0, 1, 0}, 90);
  trace::Pinhole pinhole(camera, 1);

  SECTION("ray origin is camera position") {
    geometry::Ray ray = pinhole.ray(0.5, 0.5);
    REQUIRE(ray.origin == camera.position);
  }

  SECTION("center") {
    geometry::Ray ray = pinhole.ray(0.5, 0.5);
    REQUIRE(ray.param(1).x == 0.0f);
    REQUIRE(ray.param(1).y == 0.0f);
    REQUIRE(ray.param(1).z == 0.0f);
  }
}
