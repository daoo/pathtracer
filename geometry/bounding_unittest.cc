#include <glm/glm.hpp>

#include "geometry/aabb.h"
#include "geometry/bounding.h"
#include "geometry/triangle.h"
#include "tests/catch.h"

using geometry::Aabb;
using geometry::Triangle;

TEST_CASE("single triangle, bounding is equal to triangle min/max",
          "[bounding]") {
  Triangle tri;
  tri.v0 = {0, 0, 0};
  tri.v1 = {1, 0, 0};
  tri.v2 = {1, 1, 0};
  std::vector<Triangle> tris = {tri};
  Aabb bounding = find_bounding(tris);
  REQUIRE(bounding.GetMin() == tri.GetMin());
  REQUIRE(bounding.GetMax() == tri.GetMax());
}

TEST_CASE("two equal triangles shifted in z, bounding encloses both",
          "[bounding]") {
  Triangle tria, trib;
  tria.v0 = {0, 0, 0};
  tria.v1 = {1, 0, 0};
  tria.v2 = {1, 1, 0};
  trib.v0 = {0, 0, 1};
  trib.v1 = {1, 0, 1};
  trib.v2 = {1, 1, 1};
  std::vector<Triangle> tris = {tria, trib};
  Aabb bounding = find_bounding(tris);
  REQUIRE(bounding.GetMin() == tria.GetMin());
  REQUIRE(bounding.GetMax() == trib.GetMax());
}
