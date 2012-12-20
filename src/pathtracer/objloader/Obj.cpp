#include "Obj.hpp"

#include <array>
#include <fstream>
#include <vector>

using namespace boost::filesystem;
using namespace std;

namespace objloader
{
  namespace
  {
    class ConstString
    {
      public:
        ConstString(const string& str)
          : m_str(str), m_start(0), m_end(str.size())
        {
        }

        bool operator==(const string& other) const
        {
          if (size() == other.size()) {
            for (size_t i = m_start, j = 0; i < m_end; ++i, ++j) {
              if (m_str[i] != other[j])
                return false;
            }

            return true;
          }

          return false;
        }

        size_t size() const { return m_end - m_start; }
        bool empty() const { return m_start == m_end; }

        char operator[](size_t i) const
        {
          return m_str[m_start + i];
        }

        ConstString substr(size_t start, size_t end) const
        {
          assert(start <= end);
          assert(end <= size());
          return ConstString(*this, m_start + start, m_start + end);
        }

        string str() const
        {
          return m_str.substr(m_start, size());
        }

        const char* c_str() const
        {
          return m_str.c_str() + m_start;
        }

      private:
        const string& m_str;
        const size_t m_start, m_end;

        ConstString(const ConstString& other, size_t start, size_t end)
          : m_str(other.m_str), m_start(start), m_end(end)
        {
          assert(start <= end);
          assert(end <= other.m_str.size());
        }
    };

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
    const string TOKEN_NEWMTL       = "newmtl";
    const string TOKEN_DIFFUSE1     = "diffusereflectance";
    const string TOKEN_DIFFUSE2     = "kd";
    const string TOKEN_DIFFUSE_MAP1 = "diffusereflectancemap";
    const string TOKEN_DIFFUSE_MAP2 = "map_kd";
    const string TOKEN_SPECULAR1    = "specularreflectance";
    const string TOKEN_SPECULAR2    = "ks";
    const string TOKEN_ROUGHNESS    = "specularroughness";
    const string TOKEN_EMITTANCE    = "emittance";
    const string TOKEN_TRANSPARANCY = "transparency";
    const string TOKEN_REFLECT0     = "reflat0deg";
    const string TOKEN_REFLECT90    = "reflat90deg";
    const string TOKEN_IOR          = "indexofrefraction";

    int toInt(const ConstString& str) {
      return str.empty() ? 0 : atoi(str.c_str());
    }

    void parseFacePoint(const ConstString& face, array<int, 3>& output)
    {
      const size_t end = face.size();
      array<size_t, 3> starts {{ 0, end, end }};
      array<size_t, 3> ends {{ end, end, end }};
      for (size_t i = 0, j = 0; i < end; ++i) {
        if (face[i] == '/') {
          ends[j]       = i;
          starts[j + 1] = i + 1;
          ++j;
        }
      }

      output[0] = toInt(face.substr(starts[0], ends[0]));
      output[1] = toInt(face.substr(starts[1], ends[1]));
      output[2] = toInt(face.substr(starts[2], ends[2]));
    }

    struct Token
    {
      size_t start, end;
      ConstString str;
    };

    Token getWord(const ConstString& str)
    {
      size_t i = 0;
      while (i < str.size()) {
        char c = str[i];
        if (c != ' ' || c != '\t')
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

      return { i, j, str.substr(i, j) };
    }

    template <typename T> T fill(const ConstString&);

    template <>
    Vec2 fill(const ConstString& str)
    {
      Vec2 v;
      Token tx = getWord(str);
      Token ty = getWord(str.substr(tx.end, str.size()));
      v.x = atof(tx.str.c_str());
      v.y = atof(ty.str.c_str());
      return v;
    }

    template <>
    Vec3 fill(const ConstString& str)
    {
      Vec3 v;
      Token tx = getWord(str);
      Token ty = getWord(str.substr(tx.end, str.size()));
      Token tz = getWord(str.substr(ty.end, str.size()));
      v.x = atof(tx.str.c_str());
      v.y = atof(ty.str.c_str());
      v.z = atof(tz.str.c_str());
      return v;
    }
  }

  std::ostream& operator<<(std::ostream& stream, const ObjLoaderParserException& ex) {
    stream << ex.file.string() << ":" << ex.line << ":" << ex.column
           << ": error: " << ex.message << "\n" << ex.text << "\n";
    for (size_t i = 0; i < ex.column; ++i) {
      stream << " ";
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
      if (line.empty())
        continue;

      Token tok = getWord(line);
      ConstString rest = line.substr(tok.end, line.size());

      if (tok.str.empty()) {
        throw ObjLoaderParserException(file, line_index, tok.start, line,
            "Expected token");
      }

      else if (tok.str == TOKEN_VERTEX)
        obj.vertices.push_back(fill<Vec3>(rest));

      else if (tok.str == TOKEN_NORMAL)
        obj.normals.push_back(fill<Vec3>(rest));

      else if (tok.str == TOKEN_TEXCOORD)
        obj.texcoords.push_back(fill<Vec2>(rest));

      else if (tok.str == TOKEN_FACE) {
        Token t0 = getWord(rest);
        Token t1 = getWord(line.substr(t0.end, line.size()));
        Token t2 = getWord(line.substr(t1.end, line.size()));

        array<int, 3> p0;
        array<int, 3> p1;
        array<int, 3> p2;
        parseFacePoint(t0.str, p0);
        parseFacePoint(t1.str, p1);
        parseFacePoint(t2.str, p2);

        Triangle tri
          { p0[0], p0[1], p0[2]
          , p1[0], p1[1], p1[2]
          , p2[0], p2[1], p2[2]
          };

        obj.chunks[current_chunk].triangles.push_back(tri);
      }

      else if (tok.str == TOKEN_SHADING); // Not supported
      else if (tok.str == TOKEN_GROUP); // Not supported

      else if (tok.str == TOKEN_COMMENT); // Do nothing

      else if (tok.str == TOKEN_USEMTL) {
        Token mtl = getWord(rest);
        current_chunk = obj.chunks.size();
        obj.chunks.push_back(Chunk(mtl.str.str()));
      }

      else if (tok.str == TOKEN_MTLLIB) {
        Token mtl_lib = getWord(rest);
        obj.mtl_lib = mtl_lib.str.str();
      }

      else {
        string err("Invalid token");
        err += tok.str.str();
        err += "'";

        throw ObjLoaderParserException(file, line_index, tok.start, line, err);
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

    string current_material;
    string line;
    size_t line_index = 0;
    while (getline(stream, line)) {
      if (line.empty())
        continue;

      Token tok = getWord(line);
      ConstString rest = line.substr(tok.end, line.size());
      if (tok.str.empty()) {
        throw ObjLoaderParserException(file, line_index, tok.start, line,
            "Expected token");
      }

      else if (tok.str == TOKEN_NEWMTL) {
        Token tmtl = getWord(rest);
        current_material = tmtl.str.str();
        mtl.materials[current_material] =
            { Vec3 {{0.7f}, {0.7f}, {0.7f}}
            , ""
            , Vec3 {{1.0f}, {1.0f}, {1.0f}}
            , Vec3 {{0.0f}, {0.0f}, {0.0f}}
            , 0.001f
            , 0.0f
            , 0.0f
            , 0.0f
            , 1.0f
            };
      }

      else if (tok.str == TOKEN_DIFFUSE1 || tok.str == TOKEN_DIFFUSE2) {
        //mtl.materials[current_material].diffuseReflectance = {0,0,0};
      }

      else if (tok.str == TOKEN_COMMENT); // Do nothing

      else {
      }

      ++line_index;
    }

    return mtl;
  }
}
