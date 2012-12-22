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
    const string TOKEN_MTL_SPECULAR1    = "specularreflectance";
    const string TOKEN_MTL_SPECULAR2    = "ks";
    const string TOKEN_MTL_TRANSPARANCY = "transparency";

    // Light tokens
    const string TOKEN_LIGHT_NEW       = "newlight";
    const string TOKEN_LIGHT_POSITION  = "lightposition";
    const string TOKEN_LIGHT_COLOR     = "lightcolor";
    const string TOKEN_LIGHT_RADIUS    = "lightradius";
    const string TOKEN_LIGHT_INTENSITY = "lightintensity";

    template <typename T> T parse(const string&, size_t);

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
      return {{x}, {y}};
    }

    template <>
    Vec3 parse(const string& str, size_t begin)
    {
      assert(!str.empty());

      char* end;
      float x = strtof(str.c_str() + begin, &end);
      float y = strtof(end, &end);
      float z = strtof(end, nullptr);
      return {{x}, {y}, {z}};
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
    size_t current_chunk;

    while (getline(stream, line)) {
      if (!line.empty()) {
        Word tok = getWord(line, 0);

        if (empty(tok))
          throw ObjLoaderParserException(
              file, line_index, tok.begin, line, "Expected token");

        else if (line[tok.begin] == '#');

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
          Word t0 = getWord(line, tok.end);
          Word t1 = getWord(line, t0.end);
          Word t2 = getWord(line, t0.end);

          array<int, 3> p0;
          array<int, 3> p1;
          array<int, 3> p2;
          parseFacePoint(t0, p0);
          parseFacePoint(t1, p1);
          parseFacePoint(t2, p2);

          obj.chunks[current_chunk].triangles.push_back(
            { p0[0], p0[1], p0[2]
            , p1[0], p1[1], p1[2]
            , p2[0], p2[1], p2[2]
            });
        }

        else if (equal(tok, TOKEN_SHADING)); // Not supported
        else if (equal(tok, TOKEN_GROUP)); // Not supported

        else if (equal(tok, TOKEN_COMMENT)); // Do nothing

        else if (equal(tok, TOKEN_USEMTL)) {
          Word mtl = getWord(line, tok.end);
          current_chunk = obj.chunks.size();
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

    //size_t current_camera   = 0;
    size_t current_light    = 0;
    size_t current_material = 0;

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
          ++current_material;

          Word name = getWord(line, tok.end);
          Material material =
              { str(name)
              , ""
              , Vec3 {{0.7f}, {0.7f}, {0.7f}}
              , Vec3 {{1.0f}, {1.0f}, {1.0f}}
              , Vec3 {{0.0f}, {0.0f}, {0.0f}}
              , 0.001f
              , 0.0f
              , 0.0f
              , 0.0f
              , 1.0f
              };

          mtl.materials.push_back(material);
        }

        else if (equal(tok, TOKEN_MTL_DIFFUSE))
          mtl.materials[current_material].diffuseReflectance =
            parse<Vec3>(line, tok.end);

        else if (equal(tok, TOKEN_LIGHT_NEW)) {
          ++current_light;

          Word name = getWord(line, tok.end);
          Light light =
              { str(name)
              , Vec3 {{0.0f}, {0.0f}, {0.0f}}
              , Vec3 {{1.0f}, {1.0f}, {1.0f}}
              , 0.1f
              , 10.0f
              };

          mtl.lights.push_back(light);
        }

        else if (equal(tok, TOKEN_LIGHT_POSITION))
          mtl.lights[current_light].position =
            parse<Vec3>(line, tok.end);

        else if (equal(tok, TOKEN_LIGHT_COLOR))
          mtl.lights[current_light].color =
            parse<Vec3>(line, tok.end);

        else if (equal(tok, TOKEN_LIGHT_RADIUS))
          mtl.lights[current_light].radius =
            parse<float>(line, tok.end);

        else if (equal(tok, TOKEN_LIGHT_INTENSITY))
          mtl.lights[current_light].intensity =
            parse<float>(line, tok.end);

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
