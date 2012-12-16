#include "Obj.hpp"

#include <fstream>
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
        InvalidTokenException(const string& str)
          : ObjLoaderException("Invalid token"), m_token_found(str) { }

        string m_token_found;
    };

    class InvalidValueException : public ObjLoaderException
    {
      public:
        InvalidValueException() : ObjLoaderException("Invalid value") { }
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

    ObjToken takeObjToken(istream& stream)
    {
      string str;
      while (stream.peek() != ' ') {
        str.push_back(stream.get());
      }

      if (str.empty()) throw InvalidTokenException();

      else if (str == TOKEN_VERTEX)   return TVertex;
      else if (str == TOKEN_NORMAL)   return TNormal;
      else if (str == TOKEN_TEXCOORD) return TTexCoord;
      else if (str == TOKEN_FACE)     return TFace;
      else if (str == TOKEN_SHADING)  return TShading;
      else if (str == TOKEN_MTLLIB)   return TMtlLib;
      else if (str == TOKET_USEMTL)   return TUseMtl;

      else throw InvalidTokenException(str);
    }

    string takePath(istream&)
    {
      return string();
    }

    string takeIdentifier(istream&)
    {
      return string();
    }

    float takeValue(istream& stream)
    {
      float a;
      stream >> a;
      if (stream.fail())
        throw InvalidValueException();
      return a;
    }

    template <typename T>
    void twoValues(istream& stream, vector<T>& vec)
    {
      float a = takeValue(stream);
      float b = takeValue(stream);
      vec.push_back(T{a, b});
    }

    template <typename T>
    void threeValues(istream& stream, vector<T>& vec)
    {
      float a = takeValue(stream);
      float b = takeValue(stream);
      float c = takeValue(stream);
      vec.push_back(T{a, b, c});
    }

    void nextLine(istream& stream) {
      stream.ignore(numeric_limits<streamsize>::max(), '\n');
    }
  }

  Obj loadObj(const path& file)
  {
    ifstream stream(file.string());
    Obj obj;

    size_t line_index = 0;
    while (!stream.eof()) {
      string line;
      getline(stream, line);

      size_t column = 0;

      char c = stream.peek();
      while (c != '\n') {
        if (c == ' ' || c == '\t') {
          // We ignore spaces and tabs
          stream.get();
          ++column;
        } else if (c == '#') {
          // Skip to next line
          break;
        } else {
          // Try fetching a token
          ObjToken tok = takeObjToken(stream);

          if (tok == TVertex) threeValues<Vertex>(stream, obj.m_vertices);
          else if (tok == TNormal) threeValues<Normal>(stream, obj.m_normals);
          else if (tok == TTexCoord) {
            float u = takeValue(stream);
            float w = takeValue(stream);
            obj.m_texcoords.push_back(TexCoord{u, w});
          }

          nextLine(stream);
          ++line;
        }
      }

      ++line_index;
    }

    return obj;
  }
}
