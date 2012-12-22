#include "Obj.hpp"

#include <array>
#include <fstream>

using namespace boost::filesystem;
using namespace std;

namespace objloader
{
  namespace
  {
    // Mtl tokens
    const char* TOKEN_MTL_DIFFUSE      = "kd";
    const char* TOKEN_MTL_DIFFUSE_MAP  = "map_kd";
    const char* TOKEN_MTL_EMITTANCE    = "emittance";
    const char* TOKEN_MTL_IOR          = "indexofrefraction";
    const char* TOKEN_MTL_REFLECT0     = "reflat0deg";
    const char* TOKEN_MTL_REFLECT90    = "reflat90deg";
    const char* TOKEN_MTL_ROUGHNESS    = "specularroughness";
    const char* TOKEN_MTL_SPECULAR     = "ks";
    const char* TOKEN_MTL_TRANSPARANCY = "transparency";

    // Light tokens
    const char* TOKEN_LIGHT_COLOR     = "lightcolor";
    const char* TOKEN_LIGHT_INTENSITY = "lightintensity";
    const char* TOKEN_LIGHT_POSITION  = "lightposition";
    const char* TOKEN_LIGHT_RADIUS    = "lightradius";

    // Camera tokens
    const char* TOKEN_CAMERA_FOV      = "camerafov";
    const char* TOKEN_CAMERA_POSITION = "cameraposition";
    const char* TOKEN_CAMERA_TARGET   = "cameratarget";
    const char* TOKEN_CAMERA_UP       = "cameraup";

    int parse_int(const char* str, const char** end)
    {
      int negate = 1;
      if (*str == '-') {
        ++str;
        negate = -1;
      }

      const char* ptr = str;
      while (*ptr >= '0' && *ptr <= '9') {
        ++ptr;
      }

      if (end)
        *end = ptr;
      --ptr;

      int result = 0;
      int power = 1;
      while (ptr >= str) {
        result += static_cast<int>(*ptr - 48) * power;
        power *= 10;
        --ptr;
      }

      return negate * result;
    }

    bool equal(const char* a, const char* b)
    {
      assert(a != nullptr);
      assert(*a != 0);
      assert(b != nullptr);

      while (*a != 0) {
        if (*a != *b)
          return false;

        ++a;
        ++b;
      }

      return true;
    }

    size_t skip_whitespace(const string& str, size_t begin)
    {
      size_t i = begin;
      char c;
      while (i < str.size()) {
        c = str[i];
        if (c != ' ' && c != '\t')
          break;
        ++i;
      }

      return i;
    }

    size_t next_word(const string& str, size_t begin)
    {
      size_t i = begin;
      char c;

      // If we point at a word we skip it first
      while (i < str.size()) {
        c = str[i];
        if (c == ' ' || c == '\t')
          break;
        ++i;
      }

      // Then skip any whitespace to the next word
      return skip_whitespace(str, i);
    }

    template <typename T> T parse(const string&, size_t);

    template <>
    string parse(const string& str, size_t begin)
    {
      assert(!str.empty());
      assert(begin < str.size());
      return str.substr(begin);
    }

    template <>
    float parse(const string& str, size_t begin)
    {
      assert(!str.empty());
      assert(begin < str.size());
      return strtof(str.c_str() + begin, nullptr);
    }

    template <>
    Vec2 parse(const string& str, size_t begin)
    {
      assert(!str.empty());
      assert(begin < str.size());

      char* end;
      float x = strtof(str.c_str() + begin, &end);
      float y = strtof(end, nullptr);
      return {x, y};
    }

    template <>
    Vec3 parse(const string& str, size_t begin)
    {
      assert(!str.empty());
      assert(begin < str.size());

      char* end;
      float x = strtof(str.c_str() + begin, &end);
      float y = strtof(end, &end);
      float z = strtof(end, nullptr);
      return {x, y, z};
    }

    template <>
    Point parse(const string& str, size_t begin)
    {
      assert(!str.empty());
      assert(begin < str.size());

      const char* end1;
      const char* end2;
      int v = parse_int(str.c_str() + begin, &end1);
      int t = parse_int(end1 + 1, &end2);
      int n = parse_int(end2 + 1, nullptr);
      return {static_cast<int>(v), static_cast<int>(t), static_cast<int>(n)};
    }

    template <>
    Triangle parse(const string& str, size_t begin)
    {
      assert(!str.empty());
      assert(begin < str.size());

      size_t t0_start = next_word(str, begin);
      size_t t1_start = next_word(str, t0_start);
      size_t t2_start = next_word(str, t1_start);

      return
        { parse<Point>(str, t0_start)
        , parse<Point>(str, t1_start)
        , parse<Point>(str, t2_start) };
    }
  }

  std::ostream& operator<<(std::ostream& stream,
      const ObjLoaderParserException& ex)
  {
    stream << ex.file.string() << ':' << ex.line << ':' << ex.column
           << ": error: " << ex.message << '\n' << ex.text << '\n';
    for (size_t i = 0; i < ex.column; ++i) {
      if (ex.text[i] == '\t')
        stream << '\t';
      else
        stream << ' ';
    }
    stream << "^\n";

    return stream;
  }

  Obj loadObj(const path& file)
  {
    ifstream stream(file.string());
    if (!stream.good()) {
      string err = "Failed opening file '";
      err += file.string();
      err += "'";
      throw ObjLoaderException(err);
    }

    string line;
    size_t line_index = 0;

    Obj obj;

    while (getline(stream, line)) {
      if (!line.empty()) {
        size_t offset = skip_whitespace(line, 0);

        if (offset >= line.size())
          throw ObjLoaderParserException(
              file, line_index, 0, line, "Expected token");

        char first = line[offset];

        if (first == '#');

        else if (first == 'v') {
          char second = line[offset + 1];
          if (second == ' ') {
            obj.vertices.push_back(parse<Vec3>(line, offset + 1));
          } else if (second == 'n') {
            obj.normals.push_back(parse<Vec3>(line, offset + 1));
          } else if (second == 't') {
            obj.texcoords.push_back(parse<Vec2>(line, offset + 1));
          } else {
            throw ObjLoaderParserException(
                file, line_index, offset + 1, line, "Expected v, vt or vn");
          }
        }

        else if (first == 'f') {
          if (obj.chunks.empty())
            throw ObjLoaderParserException(
                file, line_index, offset, line, "No chunk created");

          obj.chunks.back().triangles.push_back(parse<Triangle>(line, offset + 1));
        }

        else if (equal("usemtl", line.c_str() + offset)) {
          obj.chunks.push_back(Chunk(line.substr(skip_whitespace(line, offset + 7))));
        }

        else if (equal("mtllib", line.c_str() + offset)) {
          obj.mtl_lib = line.substr(skip_whitespace(line, offset + 7));
        }

        else {
          throw ObjLoaderParserException(
              file, line_index, offset, line, "Invalid token");
        }
      }

      ++line_index;
    }

    return obj;
  }

  Mtl loadMtl(const path& file)
  {
    ifstream stream(file.string());
    if (!stream.good()) {
      string err = "Failed opening file '";
      err += file.string();
      err += "'";
      throw ObjLoaderException(err);
    }

    Mtl mtl;

    string line;
    size_t line_index = 0;
    while (getline(stream, line)) {
      if (!line.empty()) {
        size_t offset = skip_whitespace(line, 0);

        if (offset >= line.size())
          throw ObjLoaderParserException(
              file, line_index, 0, line, "Expected token");

        const char* str  = line.c_str() + offset;
        const char first = line[offset];

        if (first == '#'); // Do nothing

        else if (first == 'n') {
          const char* str_next = str + 1;
          if (equal("ewmtl", str_next)) {
            mtl.materials.push_back(
                { line.substr(skip_whitespace(line, offset + 7))
                , ""
                , Vec3 {0.7f, 0.7f, 0.7f}
                , Vec3 {1.0f, 1.0f, 1.0f}
                , Vec3 {0.0f, 0.0f, 0.0f}
                , 0.001f
                , 0.0f
                , 0.0f
                , 0.0f
                , 1.0f
                });
          }

          else if (equal("ewlight", str_next)) {
            Light light =
                { Vec3 {0.0f, 0.0f, 0.0f}
                , Vec3 {1.0f, 1.0f, 1.0f}
                , 0.1f
                , 10.0f
                };

            mtl.lights.push_back(light);
          }

          else if (equal("ewcamera", str_next)) {
            Camera camera =
                { Vec3 {7.0f, 5.0f, 6.0f}
                , Vec3 {0.0f, 0.0f, 0.0f}
                , Vec3 {0.0f, 1.0f, 0.0f}
                , 10.0f
                };

            mtl.cameras.push_back(camera);
          }
        }

#define TOKEN_VALUE(list, token, type, param, error) \
  else if (equal(token, str)) { \
    if (list.empty()) \
      throw ObjLoaderParserException( \
        file, line_index, offset, line, (error)); \
    list.back().param = \
      parse<type>(line, offset); \
  }

        TOKEN_VALUE(mtl.materials , TOKEN_MTL_DIFFUSE      , Vec3   , diffuse      , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_DIFFUSE_MAP  , string , diffuseMap   , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_EMITTANCE    , Vec3   , emittance    , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_IOR          , float  , ior          , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_REFLECT0     , float  , reflAt0Deg   , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_REFLECT90    , float  , reflAt90Deg  , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_ROUGHNESS    , float  , roughness    , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_SPECULAR     , Vec3   , specular     , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_TRANSPARANCY , float  , transparency , "No material created")

        TOKEN_VALUE(mtl.lights , TOKEN_LIGHT_COLOR     , Vec3  , color     , "No light created")
        TOKEN_VALUE(mtl.lights , TOKEN_LIGHT_INTENSITY , float , intensity , "No light created")
        TOKEN_VALUE(mtl.lights , TOKEN_LIGHT_POSITION  , Vec3  , position  , "No light created")
        TOKEN_VALUE(mtl.lights , TOKEN_LIGHT_RADIUS    , float , radius    , "No light created")

        TOKEN_VALUE(mtl.cameras , TOKEN_CAMERA_FOV      , float , fov      , "No camera created")
        TOKEN_VALUE(mtl.cameras , TOKEN_CAMERA_POSITION , Vec3  , position , "No camera created")
        TOKEN_VALUE(mtl.cameras , TOKEN_CAMERA_TARGET   , Vec3  , target   , "No camera created")
        TOKEN_VALUE(mtl.cameras , TOKEN_CAMERA_UP       , Vec3  , up       , "No camera created")

#undef TOKEN_VALUE

        else {
          throw ObjLoaderParserException(
              file, line_index, offset, line, "Invalid token");
        }
      }

      ++line_index;
    }

    return mtl;
  }
}
