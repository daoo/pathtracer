#include <glm/glm.hpp>

#include "kdtree/sah_cost.h"
#include "kdtree/stream.h"
#include "third_party/catch/catch.h"

using geometry::Aabb;
using geometry::Aap;
using geometry::Axis;
using kdtree::CalculateSahCost;
using kdtree::Cost;

TEST_CASE("probability and count examples", "[sah_cost]") {
  REQUIRE(CalculateSahCost(0.5, 0.5, 1, 1) == 1.1f);
  REQUIRE(CalculateSahCost(0.5, 0.5, 1, 10) == 5.6f);
  REQUIRE(CalculateSahCost(0.5, 0.5, 10, 1) == 5.6f);
  REQUIRE(CalculateSahCost(0.25, 0.75, 1, 10) == 7.85f);
  REQUIRE(CalculateSahCost(0.25, 0.75, 10, 1) == 3.35f);
  REQUIRE(CalculateSahCost(0.75, 0.25, 1, 10) == 3.35f);
  REQUIRE(CalculateSahCost(0.75, 0.25, 10, 1) == 7.85f);
}

TEST_CASE("unit cube split in middle same cost for all axes", "[sah_cost]") {
  Aabb box{{0, 0, 0}, {0.5, 0.5, 0.5}};
  Axis axis = GENERATE(geometry::X, geometry::Y, geometry::Z);
  Aap plane(axis, 0);
  size_t left_count = 1;
  size_t right_count = 1;
  size_t plane_count = 0;

  auto cost =
      CalculateSahCost(box, plane, left_count, right_count, plane_count);

  REQUIRE(cost.cost == Approx(1.43333f));
}
