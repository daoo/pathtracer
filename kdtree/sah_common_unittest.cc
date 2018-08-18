#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "kdtree/sah_common.h"
#include "tests/catch.h"

using geometry::Aabb;
using geometry::Aap;
using kdtree::CalculateCost;

TEST_CASE("CalculateCost zero on one side puts plane on other", "[SAH]") {
  Aabb box({0, 0, 0}, {0.5, 0.5, 0.5});
  Aap plane(geometry::X, 0);

  REQUIRE(CalculateCost(box, plane, 1, 0, 1).side == kdtree::LEFT);
  REQUIRE(CalculateCost(box, plane, 0, 1, 1).side == kdtree::RIGHT);
}

TEST_CASE("CalculateCost even split puts plane on left", "[SAH]") {
  Aabb box({0, 0, 0}, {0.5, 0.5, 0.5});
  Aap plane(geometry::X, 0);

  REQUIRE(CalculateCost(box, plane, 1, 1, 1).side == kdtree::LEFT);
  REQUIRE(CalculateCost(box, plane, 2, 1, 1).side == kdtree::LEFT);
  REQUIRE(CalculateCost(box, plane, 1, 0, 1).side == kdtree::LEFT);
  REQUIRE(CalculateCost(box, plane, 1, 1, 1).side == kdtree::LEFT);
  REQUIRE(CalculateCost(box, plane, 1, 2, 1).side == kdtree::LEFT);
}
