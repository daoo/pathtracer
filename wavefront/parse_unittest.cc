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

TEST_CASE("skip zero", "[wavefront]") {
  wavefront::Parse parse("apa");
  parse.Skip(0);
  REQUIRE(parse.ParseString() == "apa");
}

TEST_CASE("skip one", "[wavefront]") {
  wavefront::Parse parse("apa");
  parse.Skip(1);
  REQUIRE(parse.ParseString() == "pa");
}

TEST_CASE("skip three", "[wavefront]") {
  wavefront::Parse parse("apa");
  parse.Skip(3);
  REQUIRE(parse.AtEnd());
}

TEST_CASE("parse float zero", "[wavefront]") {
  wavefront::Parse parse("0");
  REQUIRE(parse.ParseFloat() == 0.0f);
  REQUIRE(parse.AtEnd());
}

TEST_CASE("parse float zero decimal", "[wavefront]") {
  wavefront::Parse parse("0.0");
  REQUIRE(parse.ParseFloat() == 0.0f);
  REQUIRE(parse.AtEnd());
}

TEST_CASE("parse float 1.5", "[wavefront]") {
  wavefront::Parse parse("1.5");
  REQUIRE(parse.ParseFloat() == 1.5f);
  REQUIRE(parse.AtEnd());
}

TEST_CASE("parse float -1.5", "[wavefront]") {
  wavefront::Parse parse("-1.5");
  REQUIRE(parse.ParseFloat() == -1.5f);
  REQUIRE(parse.AtEnd());
}

TEST_CASE("parse vec2", "[wavefront]") {
  wavefront::Parse parse("1.5 1.5");
  REQUIRE(parse.ParseVec2() == glm::vec2(1.5f, 1.5f));
  REQUIRE(parse.AtEnd());
}

TEST_CASE("parse vec2 multiple space", "[wavefront]") {
  wavefront::Parse parse("1.5  1.5");
  REQUIRE(parse.ParseVec2() == glm::vec2(1.5f, 1.5f));
  REQUIRE(parse.AtEnd());
}

TEST_CASE("parse vec3", "[wavefront]") {
  wavefront::Parse parse("1.5 2.5 3.5");
  REQUIRE(parse.ParseVec3() == glm::vec3(1.5f, 2.5f, 3.5f));
  REQUIRE(parse.AtEnd());
}

TEST_CASE("parse vec3 multiple space", "[wavefront]") {
  wavefront::Parse parse("1.5  2.5  3.5");
  REQUIRE(parse.ParseVec3() == glm::vec3(1.5f, 2.5f, 3.5f));
  REQUIRE(parse.AtEnd());
}
