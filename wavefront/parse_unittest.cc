#include "tests/catch.h"
#include "wavefront/parse.h"

TEST_CASE("parse integer zero", "[wavefront]") {
  const char* str = "0";
  const char* end = nullptr;
  int value = wavefront::parse_int(str, &end);
  REQUIRE(value == 0);
  REQUIRE(end == str + 1);
}

TEST_CASE("parse positive integer", "[wavefront]") {
  const char* str = "1";
  const char* end = nullptr;
  int value = wavefront::parse_int(str, &end);
  REQUIRE(value == 1);
  REQUIRE(end == str + 1);
}

TEST_CASE("parse negative integer", "[wavefront]") {
  const char* str = "-1";
  const char* end = nullptr;
  int value = wavefront::parse_int(str, &end);
  REQUIRE(value == -1);
  REQUIRE(end == str + 2);
}

TEST_CASE("parse largish integer", "[wavefront]") {
  const char* str = "12345";
  const char* end = nullptr;
  int value = wavefront::parse_int(str, &end);
  REQUIRE(value == 12345);
  REQUIRE(end == str + 5);
}

TEST_CASE("equal empty strings", "[wavefront]") {
  REQUIRE(wavefront::equal("", ""));
}

TEST_CASE("equal example strings", "[wavefront]") {
  REQUIRE(wavefront::equal("a", "a"));
}

TEST_CASE("equal different strings", "[wavefront]") {
  REQUIRE(!wavefront::equal("a", "b"));
}

TEST_CASE("equal first string longer", "[wavefront]") {
  REQUIRE(!wavefront::equal("a", ""));
}

TEST_CASE("equal second string longer", "[wavefront]") {
  REQUIRE(!wavefront::equal("", "a"));
}

TEST_CASE("skip whitespace empty", "[wavefront]") {
  const char* str = "";
  REQUIRE(wavefront::skip_whitespace(str) == str);
}

TEST_CASE("skip whitespace one space", "[wavefront]") {
  const char* str = " ";
  REQUIRE(wavefront::skip_whitespace(str) == str + 1);
}

TEST_CASE("skip whitespace one tab", "[wavefront]") {
  const char* str = "\t";
  REQUIRE(wavefront::skip_whitespace(str) == str + 1);
}

TEST_CASE("skip whitespace several", "[wavefront]") {
  const char* str = " \t \t ";
  REQUIRE(wavefront::skip_whitespace(str) == str + 5);
}

TEST_CASE("skip whitespace non-whitespace", "[wavefront]") {
  const char* str = "apa";
  REQUIRE(wavefront::skip_whitespace(str) == str);
}
