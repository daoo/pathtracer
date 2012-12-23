#include "objloader.hpp"

#include <exception>
#include <fstream>

using namespace boost::filesystem;
using namespace std;

namespace objloader
{
  namespace
  {
    // Mtl tokens
    const char TOKEN_MTL_DIFFUSE[]      = "kd";
    const char TOKEN_MTL_DIFFUSE_MAP[]  = "map_kd";
    const char TOKEN_MTL_EMITTANCE[]    = "emittance";
    const char TOKEN_MTL_IOR[]          = "indexofrefraction";
    const char TOKEN_MTL_REFLECT0[]     = "reflat0deg";
    const char TOKEN_MTL_REFLECT90[]    = "reflat90deg";
    const char TOKEN_MTL_ROUGHNESS[]    = "specularroughness";
    const char TOKEN_MTL_SPECULAR[]     = "ks";
    const char TOKEN_MTL_TRANSPARANCY[] = "transparency";

    // Light tokens
    const char TOKEN_LIGHT_COLOR[]     = "lightcolor";
    const char TOKEN_LIGHT_INTENSITY[] = "lightintensity";
    const char TOKEN_LIGHT_POSITION[]  = "lightposition";
    const char TOKEN_LIGHT_RADIUS[]    = "lightradius";

    // Camera tokens
    const char TOKEN_CAMERA_FOV[]      = "camerafov";
    const char TOKEN_CAMERA_POSITION[] = "cameraposition";
    const char TOKEN_CAMERA_TARGET[]   = "cameratarget";
    const char TOKEN_CAMERA_UP[]       = "cameraup";

    struct Point { int v, t, n; };
    struct Face { Point p1, p2, p3; };

    template <typename T>
    T index(vector<T>& v, int i)
    {
      if (i == 0)
        return T();
      else if (i < 0)
        return v[v.size() + i];
      else
        return v[i - 1];
    }

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

    const char* skip_whitespace(const char* str)
    {
      while (*str == ' ' || *str == '\t') {
        ++str;
      }
      return str;
    }

    const char* next_word(const char* str)
    {
      // If we point at a word we skip it first
      while (*str != ' ' && *str != '\t') {
        ++str;
      }

      // Then skip any whitespace to the next word
      return skip_whitespace(str);
    }

    string parse_string(const char* str)
    {
      assert(str != nullptr);
      return string(str);
    }

    float parse_float(const char* str)
    {
      assert(str != nullptr);
      return strtof(str, nullptr);
    }

    Vec2 parse_vec2(const char* str)
    {
      assert(str != nullptr);

      char* end;
      float x = strtof(str, &end);
      float y = strtof(end, nullptr);
      return {x, y};
    }

    Vec3 parse_vec3(const char* str)
    {
      assert(str != nullptr);

      char* end;
      float x = strtof(str, &end);
      float y = strtof(end, &end);
      float z = strtof(end, nullptr);
      return {x, y, z};
    }

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
    vector<Vec3> vertices;
    vector<Vec3> normals;
    vector<Vec2> texcoords;

    while (getline(stream, line)) {
      if (!line.empty()) {
        const char* token = skip_whitespace(line.c_str());

        char first = *token;

        if (first == 'v') {
          char second = *(token + 1);
          if (second == ' ') {
            vertices.push_back(parse_vec3(token + 1));
          } else if (second == 'n') {
            normals.push_back(parse_vec3(token + 2));
          } else if (second == 't') {
            texcoords.push_back(parse_vec2(token + 2));
          }
        }

        else if (first == 'f') {
          Face face = parse_face(token + 1);

          Triangle tri;
          tri.v1.p = index(vertices  , face.p1.v);
          tri.v2.p = index(vertices  , face.p2.v);
          tri.v3.p = index(vertices  , face.p3.v);
          tri.v1.n = index(normals   , face.p1.n);
          tri.v2.n = index(normals   , face.p2.n);
          tri.v3.n = index(normals   , face.p3.n);
          tri.v1.t = index(texcoords , face.p1.t);
          tri.v2.t = index(texcoords , face.p2.t);
          tri.v3.t = index(texcoords , face.p3.t);

          obj.chunks.back().triangles.push_back(tri);
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

  Mtl loadMtl(const path& file)
  {
    ifstream stream(file.string());
    if (!stream.good()) {
      string err = "Failed opening file '";
      err += file.string();
      err += "'";
      throw runtime_error(err);
    }

    Mtl mtl;

    string line;
    unsigned int line_index = 0;
    while (getline(stream, line)) {
      if (!line.empty()) {
        const char* token = skip_whitespace(line.c_str());

        if (equal("newmtl", token)) {
          mtl.materials.push_back(
              { parse_string(skip_whitespace(token + 7))
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

        else if (equal("newlight", token)) {
          mtl.lights.push_back(
            { Vec3 {0.0f, 0.0f, 0.0f}
            , Vec3 {1.0f, 1.0f, 1.0f}
            , 0.1f
            , 10.0f
            });
        }

        else if (equal("newcamera", token)) {
          mtl.cameras.push_back(
            { Vec3 {7.0f, 5.0f, 6.0f}
            , Vec3 {0.0f, 0.0f, 0.0f}
            , Vec3 {0.0f, 1.0f, 0.0f}
            , 10.0f
            });
        }

#define TOKEN_VALUE(list, constant, parse, param, error) \
  else if (equal(constant, token)) { \
    list.back().param = parse(token + sizeof(constant)); \
  }

        TOKEN_VALUE(mtl.materials , TOKEN_MTL_DIFFUSE      , parse_vec3   , diffuse      , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_DIFFUSE_MAP  , parse_string , diffuseMap   , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_EMITTANCE    , parse_vec3   , emittance    , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_IOR          , parse_float  , ior          , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_REFLECT0     , parse_float  , reflAt0Deg   , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_REFLECT90    , parse_float  , reflAt90Deg  , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_ROUGHNESS    , parse_float  , roughness    , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_SPECULAR     , parse_vec3   , specular     , "No material created")
        TOKEN_VALUE(mtl.materials , TOKEN_MTL_TRANSPARANCY , parse_float  , transparency , "No material created")

        TOKEN_VALUE(mtl.lights , TOKEN_LIGHT_COLOR     , parse_vec3  , color     , "No light created")
        TOKEN_VALUE(mtl.lights , TOKEN_LIGHT_INTENSITY , parse_float , intensity , "No light created")
        TOKEN_VALUE(mtl.lights , TOKEN_LIGHT_POSITION  , parse_vec3  , position  , "No light created")
        TOKEN_VALUE(mtl.lights , TOKEN_LIGHT_RADIUS    , parse_float , radius    , "No light created")

        TOKEN_VALUE(mtl.cameras , TOKEN_CAMERA_FOV      , parse_float , fov      , "No camera created")
        TOKEN_VALUE(mtl.cameras , TOKEN_CAMERA_POSITION , parse_vec3  , position , "No camera created")
        TOKEN_VALUE(mtl.cameras , TOKEN_CAMERA_TARGET   , parse_vec3  , target   , "No camera created")
        TOKEN_VALUE(mtl.cameras , TOKEN_CAMERA_UP       , parse_vec3  , up       , "No camera created")

#undef TOKEN_VALUE
      }

      ++line_index;
    }

    return mtl;
  }
}
