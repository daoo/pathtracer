#include <glm/glm.hpp>

#include "geometry/aabb.h"
#include "geometry/tribox.h"
#include "tests/catch.h"

TEST_CASE("tribox contained", "[tribox]") {
  geometry::Aabb aabb({1, 1, 1}, {0.5, 0.5, 0.5});
  glm::vec3 v0(1, 1, 1);
  glm::vec3 v1(2, 1, 1);
  glm::vec3 v2(1, 2, 1);
  REQUIRE(tri_box_overlap(aabb, v0, v1, v2));
}

TEST_CASE("tribox plane intersection", "[tribox]") {
  geometry::Aabb aabb({1, 1, 1}, {1, 1, 1});
  glm::vec3 v0(1, 0.5, 1);
  glm::vec3 v1(3, 0.5, 1);
  glm::vec3 v2(3, 1, 1);
  REQUIRE(tri_box_overlap(aabb, v0, v1, v2));
}

TEST_CASE("tribox line intersection", "[tribox]") {
  geometry::Aabb aabb({1, 1, 1}, {1, 1, 1});
  glm::vec3 v0(1, 2, 1);
  glm::vec3 v1(2, 2, 1);
  glm::vec3 v2(2, 3, 1);
  REQUIRE(tri_box_overlap(aabb, v0, v1, v2));
}

TEST_CASE("tribox point intersection", "[tribox]") {
  geometry::Aabb aabb({1, 1, 1}, {1, 1, 1});
  glm::vec3 v0(2, 1, 1);
  glm::vec3 v1(3, 1, 1);
  glm::vec3 v2(3, 2, 1);
  REQUIRE(tri_box_overlap(aabb, v0, v1, v2));
}

TEST_CASE("tribox no intersection", "[tribox]") {
  geometry::Aabb aabb({1, 1, 1}, {1, 1, 1});
  glm::vec3 v0(4, 1, 1);
  glm::vec3 v1(5, 1, 1);
  glm::vec3 v2(5, 2, 1);
  REQUIRE(!tri_box_overlap(aabb, v0, v1, v2));
}
