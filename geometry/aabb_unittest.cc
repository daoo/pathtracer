#include <glm/glm.hpp>

#include "geometry/aabb.h"
#include "third_party/catch/catch.h"

TEST_CASE("aabb surface area", "[aabb]") {
  REQUIRE(geometry::Aabb::Unit().GetSurfaceArea() == 6.0f);
}

TEST_CASE("aabb min", "[aabb]") {
  REQUIRE(geometry::Aabb::Unit().GetMin() == glm::vec3(-0.5, -0.5, -0.5));
}

TEST_CASE("aabb max", "[aabb]") {
  REQUIRE(geometry::Aabb::Unit().GetMax() == glm::vec3(0.5, 0.5, 0.5));
}

TEST_CASE("aabb translate size preserved", "[aabb]") {
  geometry::Aabb box = geometry::Aabb::Unit();
  geometry::Aabb translated = box.Translate(glm::vec3(1, 1, 1));
  REQUIRE(translated.GetHalf() == box.GetHalf());
}

TEST_CASE("aabb translate center moved", "[aabb]") {
  geometry::Aabb box = geometry::Aabb::Unit();
  geometry::Aabb translated = box.Translate(glm::vec3(1, 1, 1));
  REQUIRE(translated.GetCenter() == glm::vec3(1.0, 1.0, 1.0));
}

TEST_CASE("aabb enlarge size larger", "[aabb]") {
  geometry::Aabb box = geometry::Aabb::Unit();
  geometry::Aabb enlarged = box.Enlarge(glm::vec3(0.5, 0.5, 0.5));
  REQUIRE(enlarged.GetHalf() == glm::vec3(1.0, 1.0, 1.0));
}

TEST_CASE("aabb enlarge center preserved", "[aabb]") {
  geometry::Aabb box = geometry::Aabb::Unit();
  geometry::Aabb enlarged = box.Enlarge(glm::vec3(0.5, 0.5, 0.5));
  REQUIRE(enlarged.GetCenter() == box.GetCenter());
}
