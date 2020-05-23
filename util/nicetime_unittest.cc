#include <sstream>

#include "third_party/catch/catch.h"
#include "util/nicetime.h"

std::string split(size_t seconds) {
  std::stringstream ss;
  ss << util::TimeSplit(seconds);
  return ss.str();
}

TEST_CASE("TimeSplit operator<< tests") {
  REQUIRE(split(0) == "00:00:00");
  REQUIRE(split(1) == "00:00:01");
  REQUIRE(split(60) == "00:01:00");
  REQUIRE(split(61) == "00:01:01");
  REQUIRE(split(3600) == "01:00:00");
  REQUIRE(split(3601) == "01:00:01");
  REQUIRE(split(3660) == "01:01:00");
  REQUIRE(split(3661) == "01:01:01");
  REQUIRE(split(99*3600) == "99:00:00");
}

std::string auto_unit(double seconds) {
  std::stringstream ss;
  ss << util::TimeAutoUnit(seconds);
  return ss.str();
}

TEST_CASE("TimeAutoUnit operator<< tests") {
  REQUIRE(auto_unit(0) == "0µs");
  REQUIRE(auto_unit(0.000001) == "1µs");
  REQUIRE(auto_unit(0.001) == "1000µs");
  REQUIRE(auto_unit(0.002) == "2ms");
  REQUIRE(auto_unit(1) == "1000ms");
  REQUIRE(auto_unit(2) == "2s");
  REQUIRE(auto_unit(60) == "60s");
  REQUIRE(auto_unit(61) == "1.01667m");
  REQUIRE(auto_unit(3600) == "60m");
  REQUIRE(auto_unit(3601) == "1.00028h");
  REQUIRE(auto_unit(99*3600) == "99h");
}
