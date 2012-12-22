#include "Obj.hpp"

#include "pathtracer/objloader/Word.hpp"

#include <array>
#include <fstream>

using namespace boost::filesystem;
using namespace std;

namespace objloader
{
  namespace
  {
    // General tokens
    const string TOKEN_COMMENT  = "#";

    // Obj tokens
    const string TOKEN_FACE     = "f";
    const string TOKEN_GROUP    = "g";
    const string TOKEN_MTLLIB   = "mtllib";
    const string TOKEN_NORMAL   = "vn";
    const string TOKEN_SHADING  = "s";
    const string TOKEN_TEXCOORD = "vt";
    const string TOKEN_USEMTL   = "usemtl";
    const string TOKEN_VERTEX   = "v";

    // Mtl tokens
    const string TOKEN_MTL_DIFFUSE      = "kd";
    const string TOKEN_MTL_DIFFUSE_MAP  = "map_kd";
    const string TOKEN_MTL_EMITTANCE    = "emittance";
    const string TOKEN_MTL_IOR          = "indexofrefraction";
    const string TOKEN_MTL_NEW          = "newmtl";
    const string TOKEN_MTL_REFLECT0     = "reflat0deg";
    const string TOKEN_MTL_REFLECT90    = "reflat90deg";
    const string TOKEN_MTL_ROUGHNESS    = "specularroughness";
    const string TOKEN_MTL_SPECULAR     = "ks";
    const string TOKEN_MTL_TRANSPARANCY = "transparency";

    // Light tokens
    const string TOKEN_LIGHT_COLOR     = "lightcolor";
    const string TOKEN_LIGHT_INTENSITY = "lightintensity";
    const string TOKEN_LIGHT_NEW       = "newlight";
    const string TOKEN_LIGHT_POSITION  = "lightposition";
    const string TOKEN_LIGHT_RADIUS    = "lightradius";

    // Camera tokens
    const string TOKEN_CAMERA_FOV      = "camerafov";
    const string TOKEN_CAMERA_NEW      = "newcamera";
    const string TOKEN_CAMERA_POSITION = "cameraposition";
    const string TOKEN_CAMERA_TARGET   = "cameratarget";
    const string TOKEN_CAMERA_UP       = "cameraup";

    template <typename T> T parse(const string&, size_t);

    template <>
    string parse(const string& str, size_t begin)
    {
      assert(!str.empty());
      return str.substr(begin);
    }

    template <>
    float parse(const string& str, size_t begin)
    {
      assert(!str.empty());
      return strtof(str.c_str() + begin, nullptr);
    }

    template <>
    Vec2 parse(const string& str, size_t begin)
    {
      assert(!str.empty());

      char* end;
      float x = strtof(str.c_str() + begin, &end);
      float y = strtof(end, nullptr);
      return {x, y};
    }

    template <>
    Vec3 parse(const string& str, size_t begin)
    {
      assert(!str.empty());

      char* end;
      float x = strtof(str.c_str() + begin, &end);
      float y = strtof(end, &end);
      float z = strtof(end, nullptr);
      return {x, y, z};
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
        Word tok = getWord(line, 0);

        if (empty(tok))
          throw ObjLoaderParserException(
              file, line_index, tok.begin, line, "Expected token");

        else if (line[tok.begin] == '#' || equal(tok, TOKEN_COMMENT));

        else if (tok.end >= line.size())
          throw ObjLoaderParserException(
              file, line_index, tok.end, line, "Expected token");

        else if (equal(tok, TOKEN_VERTEX))
          obj.vertices.push_back(parse<Vec3>(line, tok.end));

        else if (equal(tok, TOKEN_NORMAL))
          obj.normals.push_back(parse<Vec3>(line, tok.end));

        else if (equal(tok, TOKEN_TEXCOORD))
          obj.texcoords.push_back(parse<Vec2>(line, tok.end));

        else if (equal(tok, TOKEN_FACE)) {
          if (obj.chunks.empty())
            throw ObjLoaderParserException(
                file, line_index, tok.end, line, "No chunk created");

          Word t0 = getWord(line, tok.end);
          Word t1 = getWord(line, t0.end);
          Word t2 = getWord(line, t0.end);

          array<int, 3> p0;
          array<int, 3> p1;
          array<int, 3> p2;
          parseFacePoint(t0, p0);
          parseFacePoint(t1, p1);
          parseFacePoint(t2, p2);

          obj.chunks.back().triangles.push_back(
            { p0[0], p0[1], p0[2]
            , p1[0], p1[1], p1[2]
            , p2[0], p2[1], p2[2]
            });
        }

        else if (equal(tok, TOKEN_USEMTL)) {
          Word mtl = getWord(line, tok.end);
          obj.chunks.push_back(Chunk(str(mtl)));
        }

        else if (equal(tok, TOKEN_MTLLIB)) {
          Word mtl_lib = getWord(line, tok.end);
          obj.mtl_lib = str(mtl_lib);
        }

        else {
          string err("Invalid token '");
          err += str(tok);
          err += "'";

          throw ObjLoaderParserException(
              file, line_index, tok.begin, line, err);
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
        Word tok = getWord(line, 0);

        if (tok.str.empty())
          throw ObjLoaderParserException(
              file, line_index, tok.begin, line, "Expected token");

        else if (line[tok.begin] == '#' || equal(tok, TOKEN_COMMENT)); // Do nothing

        else if (equal(tok, TOKEN_MTL_NEW)) {
          Word name = getWord(line, tok.end);
          Material material =
              { str(name)
              , ""
              , Vec3 {0.7f, 0.7f, 0.7f}
              , Vec3 {1.0f, 1.0f, 1.0f}
              , Vec3 {0.0f, 0.0f, 0.0f}
              , 0.001f
              , 0.0f
              , 0.0f
              , 0.0f
              , 1.0f
              };

          mtl.materials.push_back(material);
        }

        else if (equal(tok, TOKEN_LIGHT_NEW)) {
          Light light =
              { Vec3 {0.0f, 0.0f, 0.0f}
              , Vec3 {1.0f, 1.0f, 1.0f}
              , 0.1f
              , 10.0f
              };

          mtl.lights.push_back(light);
        }

        else if (equal(tok, TOKEN_CAMERA_NEW)) {
          Camera camera =
              { Vec3 {7.0f, 5.0f, 6.0f}
              , Vec3 {0.0f, 0.0f, 0.0f}
              , Vec3 {0.0f, 1.0f, 0.0f}
              , 10.0f
              };

          mtl.cameras.push_back(camera);
        }

#define TOKEN_VALUE(list, token, type, param, error) \
  else if (equal(tok, (token))) { \
    if (list.empty()) \
      throw ObjLoaderParserException( \
        file, line_index, tok.begin, line, (error)); \
    list.back().param = \
      parse<type>(line, tok.end); \
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
          string err("Invalid token '");
          err += str(tok);
          err += "'";

          throw ObjLoaderParserException(
              file, line_index, tok.begin, line, err);
        }
      }

      ++line_index;
    }

    return mtl;
  }
}
