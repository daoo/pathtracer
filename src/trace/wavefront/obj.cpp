#include "obj.hpp"

#include "trace/wavefront/parse.hpp"

#include <exception>
#include <fstream>
#include <ostream>

using namespace boost::filesystem;
using namespace glm;
using namespace std;

namespace trace
{
  namespace wavefront
  {
    namespace
    {
      Point parse_point(const char* str)
      {
        assert(str != nullptr);

        const char* end1;
        const char* end2;
        int v = parse_int(str, &end1);
        int t = parse_int(end1 + 1, &end2);
        int n = parse_int(end2 + 1, nullptr);

        assert(v != 0);
        assert(n != 0);

        return {v, t, n};
      }

      Face parse_face(const char* str)
      {
        assert(str != nullptr);

        const char* t0_start = next_word(str);
        const char* t1_start = next_word(t0_start);
        const char* t2_start = next_word(t1_start);

        return
          { parse_point(t0_start)
          , parse_point(t1_start)
          , parse_point(t2_start) };
      }
    }

    Obj loadObj(const path& file)
    {
      ifstream stream(file.string());
      if (!stream.good()) {
        string err = "Failed opening file '";
        err += file.string();
        err += "'";
        throw runtime_error(err);
      }

      string line;
      unsigned int line_index = 0;

      Obj obj;

      while (getline(stream, line)) {
        if (!line.empty()) {
          const char* token = skip_whitespace(line.c_str());

          char first = *token;

          if (first == 'v') {
            char second = *(token + 1);
            if (second == ' ') {
              obj.vertices.push_back(parse_vec3(token + 1));
            } else if (second == 'n') {
              obj.normals.push_back(parse_vec3(token + 2));
            } else if (second == 't') {
              obj.texcoords.push_back(parse_vec2(token + 2));
            }
          }

          else if (first == 'f') {
            obj.chunks.back().polygons.push_back(
              parse_face(token + 1));
          }

          else if (equal("usemtl", token)) {
            obj.chunks.push_back(Chunk(
              parse_string(skip_whitespace(token + 7))));
          }

          else if (equal("mtllib", token)) {
            obj.mtl_lib =
              parse_string(skip_whitespace(token + 7));
          }
        }

        ++line_index;
      }

      return obj;
    }

  }
}
