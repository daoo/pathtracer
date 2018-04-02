#include <glm/glm.hpp>

#include "kdtree/intersect.h"
#include "tests/catch.h"

geometry::Triangle triangle2(glm::vec2 p1, glm::vec2 p2, glm::vec2 p3) {
  geometry::Triangle triangle;
  triangle.v0 = {p1.x, p1.y, 0};
  triangle.v1 = {p2.x, p2.y, 0};
  triangle.v2 = {p3.x, p3.y, 0};
  return triangle;
}

TEST_CASE("intersect empty", "[intersect]") {
  std::vector<const geometry::Triangle*> triangles;
  geometry::Aap plane(geometry::X, 0.0);
  kdtree::IntersectResults result = kdtree::PartitionTriangles(triangles, plane);
  REQUIRE(result.left.empty());
  REQUIRE(result.right.empty());
}

TEST_CASE("intersect one triangle both sides", "[intersect]") {
  const geometry::Triangle tri = triangle2({-1, 0}, {0, 1}, {1, 0});
  std::vector<const geometry::Triangle*> triangles;
  triangles.push_back(&tri);
  geometry::Aap plane(geometry::X, 0.0);
  kdtree::IntersectResults result = kdtree::PartitionTriangles(triangles, plane);
  REQUIRE(result.left == std::vector<const geometry::Triangle*>{&tri});
  REQUIRE(result.right == std::vector<const geometry::Triangle*>{&tri});
}

TEST_CASE("intersect one triangle on each side", "[intersect]") {
  const geometry::Triangle tri1 = triangle2({-3, 0}, {-2, 1}, {-1, 0});
  const geometry::Triangle tri2 = triangle2({1, 0}, {2, 1}, {3, 0});
  std::vector<const geometry::Triangle*> triangles;
  triangles.push_back(&tri1);
  triangles.push_back(&tri2);
  geometry::Aap plane(geometry::X, 0.0);
  kdtree::IntersectResults result = kdtree::PartitionTriangles(triangles, plane);
  REQUIRE(result.left == std::vector<const geometry::Triangle*>{&tri1});
  REQUIRE(result.right == std::vector<const geometry::Triangle*>{&tri2});
}

TEST_CASE("intersect one triangle in plane", "[intersect]") {
  const geometry::Triangle tri = triangle2({-1, 0}, {0, 1}, {1, 0});
  std::vector<const geometry::Triangle*> triangles;
  triangles.push_back(&tri);
  geometry::Aap plane(geometry::Z, 0.0);
  kdtree::IntersectResults result = kdtree::PartitionTriangles(triangles, plane);
  REQUIRE(result.left == std::vector<const geometry::Triangle*>{&tri});
  REQUIRE(result.right == std::vector<const geometry::Triangle*>{&tri});
}
