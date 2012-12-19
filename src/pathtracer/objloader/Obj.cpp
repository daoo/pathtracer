#include "Obj.hpp"

#include <array>
#include <fstream>
#include <sstream>
#include <vector>

using namespace boost::filesystem;
using namespace std;

namespace objloader
{
  namespace
  {
    class ObjLoaderNoTokenException : public ObjLoaderException { };
    class InvalidValueException : public ObjLoaderException
    {
      public:
        InvalidValueException(long pos) : ObjLoaderException("Invalid value"),
            m_pos(pos) { }

        long m_pos;
    };

    enum ObjToken
    {
      TVertex, TNormal, TTexCoord, TFace,
      TShading,
      TMtlLib, TUseMtl
    };

    const string TOKEN_COMMENT  = "#";
    const string TOKEN_FACE     = "f";
    const string TOKEN_GROUP    = "g";
    const string TOKEN_MTLLIB   = "mtllib";
    const string TOKEN_NORMAL   = "vn";
    const string TOKEN_SHADING  = "s";
    const string TOKEN_TEXCOORD = "vt";
    const string TOKEN_USEMTL   = "usemtl";
    const string TOKEN_VERTEX   = "v";

    void parseFacePoint(const string& face, array<int, 3>& output)
    {
      array<string, 3> strs;
      for (size_t i = 0, j = 0; i < face.size(); ++i) {
        if (face[i] == '/') {
          ++j;
        } else {
          strs[j] += face[i];
        }
      }

      stringstream ss;
      for (size_t i = 0; i < 3; ++i) {
        if (strs[i].empty()) {
          output[i] = 0;
        } else {
          ss.clear();
          ss << strs[i];
          ss >> output[i];
        }
      }
    }
  }

  std::ostream& operator<<(std::ostream& stream, const ObjLoaderParserException& ex) {
    stream << ex.m_file.string() << ":" << ex.m_line << ":" << ex.m_column
           << ": error: " << ex.m_message << "\n" << ex.m_text << "\n";
    for (size_t i = 0; i < ex.m_column; ++i) {
      stream << " ";
    }
    stream << "^\n";

    return stream;
  }

  Obj loadObj(const path& file)
  {
    ifstream stream(file.string());

    string line;
    size_t line_index = 0;

    Obj obj;
    Chunk* current_chunk;

    while (getline(stream, line)) {
      if (line.empty())
        continue;

      stringstream ss(line);

      string tok;
      ss >> tok;

      size_t column_index = static_cast<size_t>(ss.tellg()) - tok.length();

      if (tok.empty()) {
        throw ObjLoaderParserException(file, line_index, column_index, line, "Expected token");
      }

      else if (tok == TOKEN_COMMENT); // Do nothing

      else if (tok == TOKEN_MTLLIB) {
        ss >> obj.m_mtl_lib;
      }

      else if (tok == TOKEN_USEMTL) {
        string mtl;
        ss >> mtl;
        obj.m_chunks.push_back(Chunk(mtl));
        current_chunk = &obj.m_chunks.back();
      }

      else if (tok == TOKEN_VERTEX) {
        Vertex v;
        ss >> v.x >> v.y >> v.z;
        obj.m_vertices.push_back(v);
      } else if (tok == TOKEN_NORMAL) {
        Normal n;
        ss >> n.x >> n.y >> n.z;
        obj.m_normals.push_back(n);
      } else if (tok == TOKEN_TEXCOORD) {
        TexCoord t;
        ss >> t.u >> t.v;
        obj.m_texcoords.push_back(t);
      }

      else if (tok == TOKEN_SHADING); // Not supported
      else if (tok == TOKEN_GROUP); // Not supported

      else if (tok == TOKEN_FACE) {
        string a, b, c;
        ss >> a >> b >> c;

        array<int, 3> p0;
        array<int, 3> p1;
        array<int, 3> p2;
        parseFacePoint(a, p0);
        parseFacePoint(b, p1);
        parseFacePoint(c, p2);

        Triangle tri
          { p0[0], p0[1], p0[2]
          , p1[0], p1[1], p1[2]
          , p2[0], p2[1], p2[2]
          };

        current_chunk->m_triangles.push_back(tri);
      }

      else {
        stringstream err;
        err << "Invalid token '" << tok << "'";

        throw ObjLoaderParserException(file, line_index, column_index, line, err.str());
      }

      ++line_index;
    }

    return obj;
  }
}
