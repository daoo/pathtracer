#define CATCH_CONFIG_MAIN
#include "tests/catch.h"

#include <vector>
#include <glm/glm.hpp>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/triangle.h"
#include "geometry/tribox.h"
#include "kdtree/util.h"

TEST_CASE("Triangle min/max", "[triangle]") {
  geometry::Triangle tri;
  tri.v0 = {1, 1, 1};
  tri.v1 = {2, 2, 2};
  tri.v2 = {3, 3, 3};
  REQUIRE(tri.GetMin() == glm::vec3(1, 1, 1));
  REQUIRE(tri.GetMax() == glm::vec3(3, 3, 3));
}

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

TEST_CASE("split box no plane intersection", "[kdtree]") {
  geometry::Aabb boundary({5, 5, 5}, {5, 5, 5});
  geometry::Aap plane(geometry::X, 5);
  geometry::Triangle tri0, tri1;
  tri0.v0 = {2, 2, 1};
  tri0.v1 = {4, 2, 1};
  tri0.v2 = {4, 4, 1};
  tri1.v0 = {6, 6, 1};
  tri1.v1 = {8, 6, 1};
  tri1.v2 = {8, 8, 1};
  std::vector<const geometry::Triangle*> triangles({&tri0, &tri1});
  kdtree::Box parent{boundary, triangles};
  kdtree::Split split = kdtree::split_box(parent, plane);
  REQUIRE(split.left.triangles.size() == 1);
  REQUIRE(split.left.triangles[0] == &tri0);
  REQUIRE(split.right.triangles.size() == 1);
  REQUIRE(split.right.triangles[0] == &tri1);
}

TEST_CASE("split box both sides intersection", "[kdtree]") {
  geometry::Aabb boundary({5, 5, 5}, {5, 5, 5});
  geometry::Aap plane(geometry::X, 5);
  geometry::Triangle tri0;
  tri0.v0 = {4, 4, 1};
  tri0.v1 = {4, 6, 1};
  tri0.v2 = {6, 5, 1};
  std::vector<const geometry::Triangle*> triangles({&tri0});
  kdtree::Box parent{boundary, triangles};
  kdtree::Split split = kdtree::split_box(parent, plane);
  REQUIRE(split.left.triangles.size() == 1);
  REQUIRE(split.left.triangles[0] == &tri0);
  REQUIRE(split.right.triangles.size() == 1);
  REQUIRE(split.right.triangles[0] == &tri0);
}

TEST_CASE("split box triangle plane intersection", "[kdtree]") {
  geometry::Aabb boundary({5, 5, 5}, {5, 5, 5});
  geometry::Aap plane(geometry::X, 5);
  geometry::Triangle tri0;
  tri0.v0 = {plane.GetDistance(), 4, 4};
  tri0.v1 = {plane.GetDistance(), 4, 6};
  tri0.v2 = {plane.GetDistance(), 5, 6};
  std::vector<const geometry::Triangle*> triangles({&tri0});
  kdtree::Box parent{boundary, triangles};
  kdtree::Split split = kdtree::split_box(parent, plane);
  REQUIRE(split.left.triangles.size() == 1);
  REQUIRE(split.left.triangles[0] == &tri0);
  REQUIRE(split.right.triangles.size() == 1);
  REQUIRE(split.right.triangles[0] == &tri0);
}

TEST_CASE("split box line plane intersection", "[kdtree]") {
  geometry::Aabb boundary({5, 5, 5}, {5, 5, 5});
  geometry::Aap plane(geometry::X, 5);
  geometry::Triangle tri0;
  tri0.v0 = {plane.GetDistance(), 5, 4};
  tri0.v1 = {plane.GetDistance(), 5, 6};
  tri0.v2 = {6, 4, 6};
  std::vector<const geometry::Triangle*> triangles({&tri0});
  kdtree::Box parent{boundary, triangles};
  kdtree::Split split = kdtree::split_box(parent, plane);
  REQUIRE(split.left.triangles.size() == 1);
  REQUIRE(split.left.triangles[0] == &tri0);
  REQUIRE(split.right.triangles.size() == 1);
  REQUIRE(split.right.triangles[0] == &tri0);
}

TEST_CASE("split box point plane intersection", "[kdtree]") {
  geometry::Aabb boundary({5, 5, 5}, {5, 5, 5});
  geometry::Aap plane(geometry::X, 5);
  geometry::Triangle tri0;
  tri0.v0 = {plane.GetDistance(), 5, 4};
  tri0.v1 = {6, 5, 6};
  tri0.v2 = {6, 4, 6};
  std::vector<const geometry::Triangle*> triangles({&tri0});
  kdtree::Box parent{boundary, triangles};
  kdtree::Split split = kdtree::split_box(parent, plane);
  REQUIRE(split.left.triangles.size() == 1);
  REQUIRE(split.left.triangles[0] == &tri0);
  REQUIRE(split.right.triangles.size() == 1);
  REQUIRE(split.right.triangles[0] == &tri0);
}
