#include "tests/catch.h"
#include "wavefront/parse.h"

TEST_CASE("parse unsigned integer zero", "[wavefront]") {
  wavefront::Parse parse("0");
  REQUIRE(parse.ParseUInt() == 0);
  REQUIRE(parse.AtEnd());
}

TEST_CASE("parse unsigned integer", "[wavefront]") {
  wavefront::Parse parse("1");
  REQUIRE(parse.ParseUInt() == 1);
  REQUIRE(parse.AtEnd());
}

TEST_CASE("parse negative integer", "[wavefront]") {
  wavefront::Parse parse("-1");
  REQUIRE(parse.ParseInt() == -1);
  REQUIRE(parse.AtEnd());
}

TEST_CASE("parse largish integer", "[wavefront]") {
  wavefront::Parse parse("12345");
  REQUIRE(parse.ParseUInt() == 12345);
  REQUIRE(parse.AtEnd());
}

TEST_CASE("match empty strings", "[wavefront]") {
  wavefront::Parse parse("");
  REQUIRE(parse.Match(""));
}

TEST_CASE("match example strings", "[wavefront]") {
  wavefront::Parse parse("a");
  REQUIRE(parse.Match("a"));
}

TEST_CASE("match different strings", "[wavefront]") {
  wavefront::Parse parse("a");
  REQUIRE(!parse.Match("b"));
}

TEST_CASE("match first string longer", "[wavefront]") {
  wavefront::Parse parse("a");
  REQUIRE(!parse.Match(""));
}

TEST_CASE("match second string longer", "[wavefront]") {
  wavefront::Parse parse("");
  REQUIRE(!parse.Match("a"));
}

TEST_CASE("skip whitespace empty", "[wavefront]") {
  wavefront::Parse parse("");
  parse.SkipWhitespace();
  REQUIRE(parse.AtEnd());
}

TEST_CASE("skip whitespace one space", "[wavefront]") {
  wavefront::Parse parse(" ");
  parse.SkipWhitespace();
  REQUIRE(parse.AtEnd());
}

TEST_CASE("skip whitespace one tab", "[wavefront]") {
  wavefront::Parse parse("\t");
  parse.SkipWhitespace();
  REQUIRE(parse.AtEnd());
}

TEST_CASE("skip whitespace several", "[wavefront]") {
  wavefront::Parse parse(" \t \t ");
  parse.SkipWhitespace();
  REQUIRE(parse.AtEnd());
}

TEST_CASE("skip whitespace non-whitespace", "[wavefront]") {
  wavefront::Parse parse("apa");
  parse.SkipWhitespace();
  REQUIRE(parse.ParseString() == "apa");
}
