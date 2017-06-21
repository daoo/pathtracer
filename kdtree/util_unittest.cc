#include <glm/glm.hpp>
#include <vector>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/triangle.h"
#include "kdtree/util.h"
#include "tests/catch.h"

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
