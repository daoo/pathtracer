#include <glm/glm.hpp>

#include <set>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/triangle.h"
#include "kdtree/sah_fast_impl.h"
#include "tests/catch.h"

using geometry::Aabb;
using geometry::Aap;
using geometry::Triangle;
using std::set;

TEST_CASE("CalculateCost zero on one side puts plane on other", "[SAH]") {
  Aabb box({0, 0, 0}, {0.5, 0.5, 0.5});
  Aap plane(geometry::X, 0);

  REQUIRE(CalculateCost(box, plane, 1, 0, 1).side == LEFT);
  REQUIRE(CalculateCost(box, plane, 0, 1, 1).side == RIGHT);
}

TEST_CASE("CalculateCost even split puts plane on left", "[SAH]") {
  Aabb box({0, 0, 0}, {0.5, 0.5, 0.5});
  Aap plane(geometry::X, 0);

  REQUIRE(CalculateCost(box, plane, 1, 1, 1).side == LEFT);
  REQUIRE(CalculateCost(box, plane, 2, 1, 1).side == LEFT);
  REQUIRE(CalculateCost(box, plane, 1, 0, 1).side == LEFT);
  REQUIRE(CalculateCost(box, plane, 1, 1, 1).side == LEFT);
  REQUIRE(CalculateCost(box, plane, 1, 2, 1).side == LEFT);
}

TEST_CASE("ListPerfectSplits example 1", "[SAH]") {
  Aabb box({0, 0, 0}, {2, 2, 2});
  Triangle triangle;
  triangle.v0 = {0, 0, 0};
  triangle.v1 = {2, 0, 0};
  triangle.v2 = {1, 1, 0};
  set<Event> splits;

  ListPerfectSplits(box, triangle, geometry::X, &splits);

  REQUIRE(splits.size() == 2);
  REQUIRE(splits.cbegin()->plane.GetAxis() == geometry::X);
  REQUIRE(splits.cbegin()->plane.GetDistance() == 0);
  REQUIRE(splits.cbegin()->type == START);
  REQUIRE(splits.crbegin()->plane.GetAxis() == geometry::X);
  REQUIRE(splits.crbegin()->plane.GetDistance() == 2);
  REQUIRE(splits.crbegin()->type == END);
}

TEST_CASE("ListPerfectSplits example planar", "[SAH]") {
  Aabb box({0, 0, 0}, {2, 2, 2});
  Triangle triangle;
  triangle.v0 = {0, 0, 0};
  triangle.v1 = {2, 0, 0};
  triangle.v2 = {1, 1, 0};
  set<Event> splits;

  ListPerfectSplits(box, triangle, geometry::Z, &splits);

  REQUIRE(splits.size() == 1);
  REQUIRE(splits.cbegin()->plane.GetAxis() == geometry::Z);
  REQUIRE(splits.cbegin()->plane.GetDistance() == 0);
  REQUIRE(splits.cbegin()->type == PLANAR);
}

TEST_CASE("CountEvents example", "[SAH]") {
  set<Event> events{{{geometry::X, 0}, START},
                    {{geometry::X, 1}, END},
                    {{geometry::X, 2}, START},
                    {{geometry::X, 3}, END}};

  REQUIRE(CountEvents(events.cbegin(), events.cend()).pplus == 1);
  REQUIRE(CountEvents(std::next(events.cbegin()), events.cend()).pminus == 1);
}
