#include "wavefront/obj.h"

#include <exception>
#include <fstream>
#include <ostream>

#include "wavefront/parse.h"

using std::experimental::filesystem::path;

namespace wavefront {
namespace {
Point parse_point(const char* str) {
  assert(str != nullptr);

  const char* end1;
  const char* end2;
  int v = parse_int(str, &end1);
  int t = parse_int(end1 + 1, &end2);
  int n = parse_int(end2 + 1, nullptr);

  return {v, t, n};
}

Face parse_face(const char* str) {
  assert(str != nullptr);

  const char* t0_start = next_word(str);
  const char* t1_start = next_word(t0_start);
  const char* t2_start = next_word(t1_start);

  return {parse_point(t0_start), parse_point(t1_start), parse_point(t2_start)};
}
}  // namespace

Obj load_obj(const path& file) {
  std::ifstream stream(file.string());
  if (!stream.good()) {
    std::string err = "Failed opening file '";
    err += file.string();
    err += "'";
    throw std::runtime_error(err);
  }

  Obj obj;
  std::string line;
  while (getline(stream, line)) {
    const char* token = skip_whitespace(line.c_str());
    char first = *token;
    if (first == 0) continue;

    if (first == 'v') {
      char second = *(token + 1);
      if (second == ' ') {
        obj.vertices.push_back(parse_vec3(token + 1));
      } else if (second == 'n') {
        obj.normals.push_back(parse_vec3(token + 2));
      } else if (second == 't') {
        obj.texcoords.push_back(parse_vec2(token + 2));
      }
    } else if (first == 'f') {
      if (obj.chunks.empty()) {
        throw std::runtime_error("must start chunk before pushing faces to it");
      }
      obj.chunks.back().polygons.push_back(parse_face(token + 1));
    } else if (equal("usemtl", token)) {
      obj.chunks.push_back(Chunk(parse_string(skip_whitespace(token + 7))));
    } else if (equal("mtllib", token)) {
      obj.mtl_lib = parse_string(skip_whitespace(token + 7));
    } else {
      throw std::runtime_error("didn't understand line");
    }
  }

  return obj;
}
}  // namespace wavefront
