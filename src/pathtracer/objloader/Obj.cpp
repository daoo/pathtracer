#include "Obj.hpp"

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
    class InvalidTokenException : public ObjLoaderException
    {
      public:
        InvalidTokenException()
          : ObjLoaderException("Invalid token"), m_token_found() { }
        InvalidTokenException(long pos, const string& str)
          : ObjLoaderException("Invalid token"), m_token_found(str),
            m_pos(pos) { }

        string m_token_found;
        long m_pos;
    };

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

    const string TOKEN_VERTEX   = "v";
    const string TOKEN_NORMAL   = "vn";
    const string TOKEN_TEXCOORD = "vt";
    const string TOKEN_FACE     = "f";
    const string TOKEN_SHADING  = "s";
    const string TOKEN_MTLLIB   = "mtllib";
    const string TOKET_USEMTL   = "usemtl";

    void skipWhitespace(istream& stream) {
      char c = stream.peek();
      while (c == ' ' || c == '\t') {
        stream.get();
        c = stream.peek();
      }
    }

    string takeIdentifier(istream& stream)
    {
      string str;

      char c = stream.peek();
      while (c != ' ' && c != '\t' && c != '\n') {
        str.push_back(c);
        stream.get();
        c = stream.peek();
      }

      return str;
    }

    ObjToken takeObjToken(istream& stream)
    {
      size_t old_pos = stream.tellg();

      string id = takeIdentifier(stream);

      if (id.empty()) throw InvalidTokenException();

      else if (id == TOKEN_VERTEX)   return TVertex;
      else if (id == TOKEN_NORMAL)   return TNormal;
      else if (id == TOKEN_TEXCOORD) return TTexCoord;
      else if (id == TOKEN_FACE)     return TFace;
      else if (id == TOKEN_SHADING)  return TShading;
      else if (id == TOKEN_MTLLIB)   return TMtlLib;
      else if (id == TOKET_USEMTL)   return TUseMtl;

      else throw InvalidTokenException(old_pos, id);
    }

    string takePath(istream&)
    {
      return string();
    }

    float takeValue(istream& stream)
    {
      long old_pos = stream.tellg();

      float a;
      stream >> a;
      if (stream.fail())
        throw InvalidValueException(old_pos);
      return a;
    }

    template <typename T>
    void twoValues(istream& stream, vector<T>& vec)
    {
      float a = takeValue(stream); skipWhitespace(stream);
      float b = takeValue(stream);
      vec.push_back(T{a, b});
    }

    template <typename T>
    void threeValues(istream& stream, vector<T>& vec)
    {
      float a = takeValue(stream); skipWhitespace(stream);
      float b = takeValue(stream); skipWhitespace(stream);
      float c = takeValue(stream);
      vec.push_back(T{a, b, c});
    }

    void nextLine(istream& stream) {
      stream.ignore(numeric_limits<streamsize>::max(), '\n');
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
    Obj obj;

    try {
      while (!stream.eof()) {
        char c = stream.peek();

        if (c == ' ' || c == '\t') {
          stream.get();
        } else if (c == '#') {
          nextLine(stream);
        } else {
          ObjToken tok = takeObjToken(stream);

          skipWhitespace(stream);

          if (tok == TMtlLib) obj.m_mtl_lib = takePath(stream);

          else if (tok == TVertex)   threeValues<Vertex>(stream, obj.m_vertices);
          else if (tok == TNormal)   threeValues<Normal>(stream, obj.m_normals);
          else if (tok == TTexCoord) twoValues<TexCoord>(stream, obj.m_texcoords);
        }
      }

      return obj;
    } catch (const InvalidTokenException& ex) {
      stream.clear();

      const long pos = ex.m_pos;

      size_t start = 0;
      size_t line  = 0;

      stream.seekg(0);
      while (stream.tellg() < pos) {
        start = stream.tellg();
        nextLine(stream);
        ++line;
      }

      size_t column = pos - start;

      stream.seekg(start);
      string text;
      getline(stream, text);

      stringstream ss;
      ss << "Invalid token '" << ex.m_token_found << "'";

      throw ObjLoaderParserException(file, line, column, text, ss.str());
    }
  }
}
