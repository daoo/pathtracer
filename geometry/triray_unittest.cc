#include <glm/glm.hpp>

#include "geometry/ray.h"
#include "geometry/triangle.h"
#include "geometry/triray.h"
#include "third_party/catch/catch.h"

TEST_CASE("triray example body", "[triray]") {
  geometry::Triangle tri;
  tri.v0 = {0, 0, 0};
  tri.v1 = {0, 1, 0};
  tri.v2 = {1, 0, 0};
  geometry::Ray ray;
  ray.origin = {0.4, 0.4, -1};
  ray.direction = {0, 0, 1};
  auto result = intersect(tri, ray);
  REQUIRE(result.has_value());
  REQUIRE(result->t == 1);
  REQUIRE(result->u == 0.4f);
  REQUIRE(result->v == 0.4f);
}

TEST_CASE("triray example corner", "[triray]") {
  geometry::Triangle tri;
  tri.v0 = {0, 0, 0};
  tri.v1 = {0, 1, 0};
  tri.v2 = {1, 0, 0};
  geometry::Ray ray;
  ray.origin = {tri.v2.x, tri.v2.y, -1};
  ray.direction = {0, 0, 1};
  auto result = intersect(tri, ray);
  REQUIRE(result.has_value());
  REQUIRE(result->t == 1);
  REQUIRE(result->u == tri.v2.y);
  REQUIRE(result->v == tri.v2.x);
}
