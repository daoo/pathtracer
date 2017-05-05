#define CATCH_CONFIG_MAIN
#include "tests/catch.h"

#include <glm/glm.hpp>

#include "geometry/triangle.h"

TEST_CASE("Triangle min/max", "[triangle]") {
  geometry::Triangle tri;
  tri.v0 = {1, 1, 1};
  tri.v1 = {2, 2, 2};
  tri.v2 = {3, 3, 3};
  REQUIRE(tri.GetMin() == glm::vec3(1, 1, 1));
  REQUIRE(tri.GetMax() == glm::vec3(3, 3, 3));
}
