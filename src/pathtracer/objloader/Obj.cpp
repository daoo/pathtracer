#include "Obj.hpp"

#include <array>
#include <fstream>
#include <vector>

#include <iostream>

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
    const string TOKEN_MTL_NEW          = "newmtl";
    const string TOKEN_MTL_DIFFUSE1     = "diffusereflectance";
    const string TOKEN_MTL_DIFFUSE2     = "kd";
    const string TOKEN_MTL_DIFFUSE_MAP1 = "diffusereflectancemap";
    const string TOKEN_MTL_DIFFUSE_MAP2 = "map_kd";
    const string TOKEN_MTL_SPECULAR1    = "specularreflectance";
    const string TOKEN_MTL_SPECULAR2    = "ks";
    const string TOKEN_MTL_ROUGHNESS    = "specularroughness";
    const string TOKEN_MTL_EMITTANCE    = "emittance";
    const string TOKEN_MTL_TRANSPARANCY = "transparency";
    const string TOKEN_MTL_REFLECT0     = "reflat0deg";
    const string TOKEN_MTL_REFLECT90    = "reflat90deg";
    const string TOKEN_MTL_IOR          = "indexofrefraction";

    // Light tokens
    const string TOKEN_LIGHT_NEW       = "newlight";
    const string TOKEN_LIGHT_POSITION  = "lightposition";
    const string TOKEN_LIGHT_COLOR     = "lightcolor";
    const string TOKEN_LIGHT_RADIUS    = "lightradius";
    const string TOKEN_LIGHT_INTENSITY = "lightintensity";

    struct Word { const string& str; size_t begin, end; };

    bool empty(const Word& word) { return word.begin == word.end; }
    size_t size(const Word& word) { return word.end - word.begin; }

    const char* c_str(const Word& word) { return word.str.c_str() + word.begin; }
    string str(const Word& word) { return word.str.substr(word.begin, size(word)); }

    bool equal(const Word& word, const string& other)
    {
      if (size(word) == other.size()) {
        size_t i = word.begin;
        size_t j = 0;
        while (i < size(word)) {
          if (word.str[i] != other[j])
            return false;
          ++i;
          ++j;
        }

        return true;
      }

      return false;
    }

    int toInt(const Word& word)
    {
      return empty(word) ? 0 : atoi(c_str(word));
    }

    float toFloat(const Word& word)
    {
      return strtof(c_str(word), nullptr);
    }

    void parseFacePoint(const Word& word, array<int, 3>& output)
    {
      assert(!empty(word));

      array<size_t, 3> starts {{ word.begin, word.end, word.end }};
      array<size_t, 3> ends {{ word.end, word.end, word.end }};
      for (size_t i = word.begin, j = 0; i < word.end; ++i) {
        if (word.str[i] == '/') {
          ends[j]       = i;
          starts[j + 1] = i + 1;
          ++j;
        }
      }

      output[0] = toInt(Word{word.str, starts[0], ends[0]});
      output[1] = toInt(Word{word.str, starts[1], ends[1]});
      output[2] = toInt(Word{word.str, starts[2], ends[2]});
    }

    Word getWord(const string& str, size_t begin)
    {
      assert(!str.empty());

      size_t i = begin;
      while (i < str.size()) {
        char c = str[i];
        if (c != ' ' && c != '\t')
          break;
        ++i;
      }

      size_t j = i;
      while (j < str.size()) {
        char c = str[j];
        if (c == ' ' || c == '\t' || c == '\n')
          break;
        ++j;
      }

      return {str, i, j};
    }

    template <typename T> T parse(const string&, size_t);

    template <>
    float parse(const string& str, size_t begin)
    {
      assert(!str.empty());

      return toFloat(getWord(str, begin));
    }

    template <>
    Vec2 parse(const string& str, size_t begin)
    {
      assert(!str.empty());

      Word tx = getWord(str, begin);
      Word ty = getWord(str, tx.end);
      return
        { {toFloat(tx)}
        , {toFloat(ty)}
        };
    }

    template <>
    Vec3 parse(const string& str, size_t begin)
    {
      assert(!str.empty());

      Word tx = getWord(str, begin);
      Word ty = getWord(str, tx.end);
      Word tz = getWord(str, ty.end);
      return
        { {toFloat(tx)}
        , {toFloat(ty)}
        , {toFloat(tz)}
        };
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

        else if (equal(tok, TOKEN_MTL_DIFFUSE1) ||
            equal(tok, TOKEN_MTL_DIFFUSE2))
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

        else if (equal(tok, TOKEN_COMMENT)); // Do nothing

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
