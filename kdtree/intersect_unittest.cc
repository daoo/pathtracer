#include <glm/glm.hpp>

#include "geometry/aap.h"
#include "geometry/aabb.h"
#include "kdtree/intersect.h"
#include "third_party/catch/catch.h"

using geometry::Aabb;
using geometry::Aap;
using geometry::Triangle;
using geometry::X;
using geometry::Y;
using geometry::Z;
using glm::vec2;
using kdtree::IntersectResults;
using kdtree::PartitionTriangles;
using std::vector;

Triangle triangle2(vec2 p1, vec2 p2, vec2 p3) {
  Triangle triangle;
  triangle.v0 = {p1.x, p1.y, 0};
  triangle.v1 = {p2.x, p2.y, 0};
  triangle.v2 = {p3.x, p3.y, 0};
  return triangle;
}

TEST_CASE("intersect empty", "[intersect]") {
  Aabb boundary = Aabb::Unit();
  vector<const Triangle*> triangles;
  Aap plane(X, 0.0);
  IntersectResults result = PartitionTriangles(boundary, triangles, plane);
  REQUIRE(result.left.empty());
  REQUIRE(result.right.empty());
}

TEST_CASE("intersect one triangle both sides", "[intersect]") {
  Aabb boundary = Aabb::FromExtents({-1, 0, 0}, {1, 1, 0});
  Triangle tri = triangle2({-1, 0}, {0, 1}, {1, 0});
  vector<const Triangle*> triangles{&tri};
  Aap plane(X, 0.0);
  IntersectResults result = PartitionTriangles(boundary, triangles, plane);
  REQUIRE(result.left == vector<const Triangle*>{&tri});
  REQUIRE(result.right == vector<const Triangle*>{&tri});
}

TEST_CASE("intersect one triangle on each side", "[intersect]") {
  Aabb boundary = Aabb::FromExtents({-3, 0, 0}, {3, 2, 0});
  Triangle tri1 = triangle2({-3, 0}, {-2, 1}, {-1, 0});
  Triangle tri2 = triangle2({1, 0}, {2, 1}, {3, 0});
  vector<const Triangle*> triangles{&tri1, &tri2};
  Aap plane(X, 0.0);
  IntersectResults result = PartitionTriangles(boundary, triangles, plane);
  REQUIRE(result.left == vector<const Triangle*>{&tri1});
  REQUIRE(result.right == vector<const Triangle*>{&tri2});
}

TEST_CASE("intersect one triangle in plane", "[intersect]") {
  Aabb boundary = Aabb::FromExtents({-1, 0, 0}, {1, 1, 0});
  Triangle tri = triangle2({-1, 0}, {0, 1}, {1, 0});
  vector<const Triangle*> triangles{&tri};
  Aap plane(Z, 0.0);
  IntersectResults result = PartitionTriangles(boundary, triangles, plane);
  REQUIRE(result.left.empty());
  REQUIRE(result.plane == vector<const Triangle*>{&tri});
  REQUIRE(result.right.empty());
}
