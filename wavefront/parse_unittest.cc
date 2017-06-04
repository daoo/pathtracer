#include "tests/catch.h"
#include "wavefront/parse.h"

TEST_CASE("zero", "[parse_int]") {
  const char* str = "0";
  const char* end = nullptr;
  int value = wavefront::parse_int(str, &end);
  REQUIRE(value == 0);
  REQUIRE(end == str + 1);
}

TEST_CASE("positive int", "[parse_int]") {
  const char* str = "1";
  const char* end = nullptr;
  int value = wavefront::parse_int(str, &end);
  REQUIRE(value == 1);
  REQUIRE(end == str + 1);
}

TEST_CASE("negative int", "[parse_int]") {
  const char* str = "-1";
  const char* end = nullptr;
  int value = wavefront::parse_int(str, &end);
  REQUIRE(value == -1);
  REQUIRE(end == str + 2);
}

TEST_CASE("largish int", "[parse_int]") {
  const char* str = "12345";
  const char* end = nullptr;
  int value = wavefront::parse_int(str, &end);
  REQUIRE(value == 12345);
  REQUIRE(end == str + 5);
}

TEST_CASE("empty", "[equal]") {
  REQUIRE(wavefront::equal("", ""));
}

TEST_CASE("example", "[equal]") {
  REQUIRE(wavefront::equal("a", "a"));
}

TEST_CASE("not equal example", "[equal]") {
  REQUIRE(!wavefront::equal("a", "b"));
}

TEST_CASE("first longer", "[equal]") {
  REQUIRE(!wavefront::equal("a", ""));
}

TEST_CASE("second longer", "[equal]") {
  REQUIRE(!wavefront::equal("", "a"));
}

TEST_CASE("empty", "[skip_whitespace]") {
  const char* str = "";
  REQUIRE(wavefront::skip_whitespace(str) == str);
}

TEST_CASE("skip one space", "[skip_whitespace]") {
  const char* str = " ";
  REQUIRE(wavefront::skip_whitespace(str) == str + 1);
}

TEST_CASE("skip one tab", "[skip_whitespace]") {
  const char* str = "\t";
  REQUIRE(wavefront::skip_whitespace(str) == str + 1);
}

TEST_CASE("skip many whites", "[skip_whitespace]") {
  const char* str = " \t \t ";
  REQUIRE(wavefront::skip_whitespace(str) == str + 5);
}

TEST_CASE("do not skip non-whitespace", "[skip_whitespace]") {
  const char* str = "apa";
  REQUIRE(wavefront::skip_whitespace(str) == str);
}
