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

    const string TOKEN_COMMENT  = "#";
    const string TOKEN_FACE     = "f";
    const string TOKEN_GROUP    = "g";
    const string TOKEN_MTLLIB   = "mtllib";
    const string TOKEN_NORMAL   = "vn";
    const string TOKEN_SHADING  = "s";
    const string TOKEN_TEXCOORD = "vt";
    const string TOKEN_USEMTL   = "usemtl";
    const string TOKEN_VERTEX   = "v";

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

    Token sub(const ConstString& str, size_t start)
    {
      size_t i = start;
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

      Token tok = sub(line, 0);
      if (tok.str.empty()) {
        throw ObjLoaderParserException(file, line_index, tok.start, line,
            "Expected token");
      }

      else if (tok.str == TOKEN_VERTEX) {
        Vertex v;
        Token tx = sub(line, tok.end);
        Token ty = sub(line, tx.end);
        Token tz = sub(line, ty.end);
        v.x = atof(tx.str.c_str());
        v.y = atof(ty.str.c_str());
        v.z = atof(tz.str.c_str());
        obj.m_vertices.push_back(v);
      }

      else if (tok.str == TOKEN_NORMAL) {
        Normal n;
        Token tx = sub(line, tok.end);
        Token ty = sub(line, tx.end);
        Token tz = sub(line, ty.end);
        n.x = atof(tx.str.c_str());
        n.y = atof(ty.str.c_str());
        n.z = atof(tz.str.c_str());
        obj.m_normals.push_back(n);

      }

      else if (tok.str == TOKEN_TEXCOORD) {
        TexCoord t;
        Token tx = sub(line, tok.end);
        Token ty = sub(line, tx.end);
        t.u = atof(tx.str.c_str());
        t.v = atof(ty.str.c_str());
        obj.m_texcoords.push_back(t);
      }

      else if (tok.str == TOKEN_FACE) {
        Token t0 = sub(line, tok.end);
        Token t1 = sub(line, t0.end);
        Token t2 = sub(line, t1.end);

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

        current_chunk->m_triangles.push_back(tri);
      }

      else if (tok.str == TOKEN_SHADING); // Not supported
      else if (tok.str == TOKEN_GROUP); // Not supported

      else if (tok.str == TOKEN_COMMENT); // Do nothing

      else if (tok.str == TOKEN_USEMTL) {
        Token mtl = sub(line, tok.end);
        obj.m_chunks.push_back(Chunk(mtl.str.str()));
        current_chunk = &obj.m_chunks.back();
      }

      else if (tok.str == TOKEN_MTLLIB) {
        Token mtl_lib = sub(line, tok.end);
        obj.m_mtl_lib = mtl_lib.str.str();
      }

      else {
        stringstream err;
        err << "Invalid token '" << tok.str.str() << "'";

        throw ObjLoaderParserException(file, line_index, tok.start, line, err.str());
      }

      ++line_index;
    }

    return obj;
  }

  Mtl loadMtl(const path& file)
  {
  }
}
