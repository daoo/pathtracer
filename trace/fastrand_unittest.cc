#include "third_party/catch/catch.h"
#include "trace/fastrand.h"

TEST_CASE("fastrand.unit in range [0, 1)", "[fastrand]") {
  trace::FastRand rand;
  for (int i = 0; i < 1000; ++i) {
    float x = rand.unit();
    REQUIRE(x >= 0.0f);
    REQUIRE(x < 1.0f);
  }
}

TEST_CASE("fastrand.range in range", "[fastrand]") {
  trace::FastRand rand;
  for (int i = 0; i < 1000; ++i) {
    float x = rand.range(1.0f, 2.0f);
    REQUIRE(x >= 1.0f);
    REQUIRE(x < 2.0f);
  }
}
