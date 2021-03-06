#include "wavefront/obj.h"

#include <exception>
#include <fstream>
#include <ostream>

#include "wavefront/parser.h"

using std::experimental::filesystem::path;

namespace wavefront {
namespace {
class ObjParser : public StringParser {
 public:
  explicit ObjParser(const std::string& str) : StringParser(str) {}

  Point ParsePoint() {
    int v = ParseInt();
    Skip(1);
    int t = ParseInt();
    Skip(1);
    int n = ParseInt();
    return {v, t, n};
  }

  Face ParseFace() {
    Point p1 = ParsePoint();
    SkipWhitespace();
    Point p2 = ParsePoint();
    SkipWhitespace();
    Point p3 = ParsePoint();
    SkipWhitespace();
    return {p1, p2, p3};
  }
};
}  // namespace

Obj ParseObj(std::ifstream& stream) {
  Obj obj;
  std::string line;
  int line_number = 0;
  while (getline(stream, line)) {
    try {
      ObjParser parse(line);
      parse.SkipWhitespace();
      if (parse.AtEnd()) continue;
      if (parse.Match("#")) continue;

      if (parse.Match("vn")) {
        parse.SkipWhitespace();
        obj.normals.push_back(parse.ParseVec3());
      } else if (parse.Match("vt")) {
        parse.SkipWhitespace();
        obj.texcoords.push_back(parse.ParseVec2());
      } else if (parse.Match("v")) {
        parse.SkipWhitespace();
        obj.vertices.push_back(parse.ParseVec3());
      } else if (parse.Match("f")) {
        if (obj.chunks.empty()) {
          throw StringException(line, line.c_str(),
                                "must start chunk before pushing faces to it");
        }
        parse.SkipWhitespace();
        obj.chunks.back().polygons.push_back(parse.ParseFace());
      } else if (parse.Match("usemtl")) {
        parse.SkipWhitespace();
        obj.chunks.push_back(Chunk(parse.ParseString()));
      } else if (parse.Match("mtllib")) {
        parse.SkipWhitespace();
        obj.mtl_lib = parse.ParseString();
      } else if (parse.Match("o")) {
        // skip object grouping
        continue;
      } else if (parse.Match("s")) {
        // skip smooth shading setting
        continue;
      } else {
        throw StringException(line, line.c_str(), "unknown expression");
      }
    } catch (const StringException& ex) {
      throw LineException(line_number, ex);
    }
    ++line_number;
  }

  return obj;
}

Obj LoadObj(const path& file) {
  std::ifstream stream(file.string());
  if (!stream.good()) {
    std::string err = "Failed opening file '";
    err += file.string();
    err += "'";
    throw std::runtime_error(err);
  }
  try {
    return ParseObj(stream);
  } catch (const LineException& ex) {
    throw FileException(file, ex);
  }
}
}  // namespace wavefront
