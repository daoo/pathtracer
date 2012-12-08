#include <algorithm>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <sstream>

#include "objmodel.hpp"

using namespace glm;
using namespace std;

namespace {
  struct ObjTri {
    int v[3];
    int t[3];
    int n[3];
  };

  // The next step is to create a dedicated lexer (flex takes about 40% of total), where we can make
  // use of the line by line structure of the file to optimize.
  class ObjLexer {
    public:
      enum { s_bufferLength = 512 };

      ObjLexer(istream* input) :
        m_input(input),
        m_bufferPos(0),
        m_bufferEnd(0)
    {
    }

      enum Tokens
      {
        T_Eof      = 0,
        T_MtlLib   = 'm' << 8 | 't',
        T_UseMtl   = 'u' << 8 | 's',
        T_Face     = 'f' << 8 | ' ', // line starts with 'f' followed by ' '
        T_Face2    = 'f' << 8 | '\t', // line starts with 'f' followed by '\t'
        T_Vertex   = 'v' << 8 | ' ', // line starts with 'v' followed by ' '
        T_Vertex2  = 'v' << 8 | '\t', // line starts with 'v' followed by '\t'
        T_Normal   = 'v' << 8 | 'n',
        T_TexCoord = 'v' << 8 | 't',
      };

      inline int fillBuffer()
      {
        if (m_bufferPos >= m_bufferEnd)
        {
          m_input->read(m_buffer, s_bufferLength);
          m_bufferEnd = int(m_input->gcount());
          m_bufferPos = 0;
        }
        return m_bufferEnd != 0;
      }

      inline int nextChar()
      {
        if (fillBuffer())
        {
          return m_buffer[m_bufferPos++];
        }
        return 0;
      }

      int firstLine()
      {
        // read the first line token.
        return nextChar() << 8 | nextChar();
      }

      inline int nextLine()
      {
        // scan to end of line...
        while('\n' != nextChar())
        {
        }
        while(matchChar('\n') || matchChar('\r'))
        {
        }
        // Or: convert next 2 chars to token (16 bit), can be mt, us, vn, vt, v , v\t, f , f\t, unknown
        return nextChar() << 8 | nextChar();

        /*    switch()
              {
              case 'm':
              return (match("tllib") && matchWs()) ? OT_MtlLib : OT_Unknown;
              case 'u':
              return (match("semtl") && matchWs()) ? OT_UseMtl : OT_Unknown;
              case 'f':
              return matchWs() ? OT_Face : OT_Unknown;
              case 'v':
              if (matchWs())
              return OT_Vertex;
              if (match("n") && matchWs())
              return OT_Normal;
              if (match("t") && matchWs())
              return OT_TexCoord;
              break;
              default:
              break;
              };
              return OT_Unknown;*/
      }


      inline bool match(const char s[], const size_t l)
      {
        for (int i = 0; fillBuffer() && i < int(l) - 1; ++i)
        {
          if (s[i] != m_buffer[m_bufferPos])
          {
            return false;
          }
          else
          {
            ++m_bufferPos;
          }
        }
        return true;
      }
      inline bool matchString(string &str)
      {
        while (fillBuffer() && !isspace(m_buffer[m_bufferPos]))
        {
          str.push_back(m_buffer[m_bufferPos++]);
        }
        return !str.empty();
      }

      inline bool matchFloat(float &result)
      {
        bool found = false;
        result = 0.0f;
        float sign = 1.0f;
        if (matchChar('-'))
        {
          sign = -1.0f;
          found = true;
        }
        char c;
        while (fillBuffer() && myIsDigit(c = m_buffer[m_bufferPos]))
        {
          result = result * 10.0f + float(c - '0');
          ++m_bufferPos;
          found = true;
        }
        float frac = 0.1f;
        if (matchChar('.'))
        {
          char c2;
          while (fillBuffer() && myIsDigit(c2 = m_buffer[m_bufferPos]))
          {
            result += frac * float(c2 - '0');
            ++m_bufferPos;
            frac *= 0.1f;
          }
          found = true;
        }
        if (matchChar('e') || matchChar('E'))
        {
          float expSign = matchChar('-') ? -1.0f : 1.0f;
          int exp = 0;
          if (matchInt(exp))
          {
            result = result * powf(10.0f, float(exp) * expSign);
          }
        }
        result *= sign;
        return found;
      }

      inline bool myIsDigit(char c)
      {
        return ((unsigned int)(c) - (unsigned int)('0') < 10U);
      }

      inline bool matchInt(int &result)
      {
        bool found = false;
        result = 0;
        char c;
        while (fillBuffer() && myIsDigit(c = m_buffer[m_bufferPos]))// isdigit(m_buffer[m_bufferPos]))
        {
          result = result * 10 + int(c - '0');
          ++m_bufferPos;
          found = true;
        }
        return found;
      }
      inline bool matchChar(int matchTo)
      {
        if (fillBuffer() && m_buffer[m_bufferPos] == matchTo)
        {
          m_bufferPos++;
          return true;
        }
        return false;
      }

      inline bool matchWs(bool optional = false)
      {
        bool found = false;
        while (fillBuffer() &&
            (m_buffer[m_bufferPos] == ' ' || m_buffer[m_bufferPos] == '\t'))
        {
          found = true;
          m_bufferPos++;
        }
        return found || optional;
      }
      istream* m_input;
      char m_buffer[s_bufferLength];
      int m_bufferPos;
      int m_bufferEnd;
  };

  inline static bool parseFaceIndSet(ObjLexer &lexer, ObjTri &t, int v) {
    t.v[v] = -1;
    t.t[v] = -1;
    t.n[v] = -1;

    if(lexer.matchWs(true)
        && lexer.matchInt(t.v[v])
        &&   lexer.matchChar('/')
        &&   (lexer.matchInt(t.t[v]) || true)  // The middle index is optional!
        &&   lexer.matchChar('/')
        &&   lexer.matchInt(t.n[v]))
    {
      // need to adjust for silly obj 1 based indexing
      t.v[v] -= 1;
      t.t[v] -= 1;
      t.n[v] -= 1;
      return true;
    }
    return false;
  }
}

void inline OBJModel::loadOBJ(ifstream& file, const string& basePath) {
  vector<vec3> positions;
  vector<vec3> normals;
  vector<vec2> uvs;
  vector<ObjTri> tris;

  positions.reserve(256 * 1024);
  normals.reserve(256 * 1024);
  uvs.reserve(256 * 1024);
  tris.reserve(256 * 1024);

  vector<pair<string, size_t> > materialChunks;

  cout << "  Reading data...\n";

  ObjLexer lexer(&file);
  for(int token = lexer.firstLine(); token != ObjLexer::T_Eof; token = lexer.nextLine())
  {
    switch(token)
    {
      case ObjLexer::T_MtlLib:
        {
          string materialFile;
          if(lexer.match("llib", sizeof("llib")) && lexer.matchWs() && lexer.matchString(materialFile))
          {
            loadMaterials(basePath + materialFile, basePath);
          }
          break;
        }
      case ObjLexer::T_UseMtl:
        {
          string materialName;
          if(lexer.match("emtl", sizeof("emtl")) && lexer.matchWs() && lexer.matchString(materialName))
          {
            if (materialChunks.size() == 0 || (*materialChunks.rbegin()).first != materialName)
            {
              materialChunks.push_back(make_pair(materialName, tris.size()));
            }
          }
        }
        break;
      case ObjLexer::T_Vertex:
      case ObjLexer::T_Vertex2:
        {
          vec3 p;
          if (lexer.matchWs(true)
              && lexer.matchFloat(p.x)
              && lexer.matchWs()
              && lexer.matchFloat(p.y)
              && lexer.matchWs()
              && lexer.matchFloat(p.z))
          {
            positions.push_back(p);
          }
        }
        break;
      case ObjLexer::T_Normal:
        {
          vec3 n { 0.0f, 0.0f, 0.0f };
          if (lexer.matchWs(true)
              && lexer.matchFloat(n.x)
              && lexer.matchWs()
              && lexer.matchFloat(n.y)
              && lexer.matchWs()
              && lexer.matchFloat(n.z))
          {
            normals.push_back(n);
          }
        }
        break;
      case ObjLexer::T_TexCoord:
        {
          vec2 t;
          if (lexer.matchWs(true)
              && lexer.matchFloat(t.x)
              && lexer.matchWs()
              && lexer.matchFloat(t.y))
          {
            uvs.push_back(t);
          }
        }
        break;
      case ObjLexer::T_Face:
      case ObjLexer::T_Face2:
        {
          ObjTri t;
          // parse vert 0 and 1
          if (parseFaceIndSet(lexer, t, 0) && parseFaceIndSet(lexer, t, 1))
          {
            // let any more produce one more triangle
            while(parseFaceIndSet(lexer, t, 2))
            {
              // kick tri,
              tris.push_back(t);
              // the make last vert second (this also keeps winding the same).
              t.n[1] = t.n[2];
              t.t[1] = t.t[2];
              t.v[1] = t.v[2];
            }
          }
        }
        break;
      default:
        break;
    };
  }

  cout << "  done.\n";

  cout << "  Shuffling..." << flush;
  // Reshuffle the normals and vertices to be unique.
  for (size_t i = 0; i < materialChunks.size(); ++i) {
    Chunk chunk;
    chunk.material = &m_materials[materialChunks[i].first];
    const size_t start = materialChunks[i].second;
    const size_t end = i + 1 < materialChunks.size() ? materialChunks[i + 1].second : tris.size();

    chunk.m_normals.resize(3 * (end - start));
    chunk.m_positions.resize(3 * (end - start));
    chunk.m_uvs.resize(3 * (end - start));

    for (size_t k = start; k < end; ++k)
    {
      for (int j = 0; j < 3; ++j)
      {
        chunk.m_normals[(k  - start) * 3 + j] = (normals[tris[k].n[j]]);
        chunk.m_positions[(k  - start) * 3 + j] = (positions[tris[k].v[j]]);
        if(tris[k].t[j] != -1)
        {
          chunk.m_uvs[(k  - start) * 3 + j] = (uvs[tris[k].t[j]]);
        }
      }
    }

#if 0
    printf("// mat %s\n", materialChunks[i].first.c_str());
    printf("vec3 normals[] = {\n");
    for (size_t index = start; index < end; ++index)
    {
      printf("{ %f, %f, %f },\n", normals[tris[index].n[0]].x, normals[tris[index].n[0]].y, normals[tris[index].n[0]].z);
      printf("{ %f, %f, %f },\n", normals[tris[index].n[1]].x, normals[tris[index].n[1]].y, normals[tris[index].n[1]].z);
      printf("{ %f, %f, %f },\n", normals[tris[index].n[2]].x, normals[tris[index].n[2]].y, normals[tris[index].n[2]].z);
    }
    printf("};\n");


    printf("vec3 positions[] = {\n");
    for (size_t index = start; index < end; ++index)
    {
      printf("{ %f, %f, %f },\n", positions[tris[index].v[0]].x, positions[tris[index].v[0]].y, positions[tris[index].v[0]].z);
      printf("{ %f, %f, %f },\n", positions[tris[index].v[1]].x, positions[tris[index].v[1]].y, positions[tris[index].v[1]].z);
      printf("{ %f, %f, %f },\n", positions[tris[index].v[2]].x, positions[tris[index].v[2]].y, positions[tris[index].v[2]].z);
    }
    printf("};\n");

    printf("vec3 texCoords[] = {\n");
    for (size_t index = start; index < end; ++index)
    {
      printf("{ %f, %f },\n", uvs[tris[index].t[0]].x, uvs[tris[index].t[0]].y);
      printf("{ %f, %f },\n", uvs[tris[index].t[1]].x, uvs[tris[index].t[1]].y);
      printf("{ %f, %f },\n", uvs[tris[index].t[2]].x, uvs[tris[index].t[2]].y);
    }
    printf("};\n");
#endif
    m_chunks.push_back(chunk);
  }
  cout << " done.\n";

  // lastly we could look out for duplicates and compact the array down again, if we would.
}

void OBJModel::loadMaterials(const string& fileName, const string&) {
  ifstream file;
  file.open(fileName.c_str());
  if (!file) {
    stringstream ss;
    ss << "Error in openening file '" << fileName << "'\n";
    throw ss.str();
  }
  string currentMaterial("");
  string currentLight("");
  string currentCamera("");

  string lineread;
  while(getline(file, lineread)) // Read line by line
  {
    //cout << lineread << "\n";
    stringstream ss(lineread);
    string firstword;
    ss >> firstword;
    transform(firstword.begin(), firstword.end(), firstword.begin(), (int (*)(int))std::tolower);

    if(firstword == "newmtl")
    {
      ss >> currentMaterial;
      Material m =
      {
        vec3(0.7f, 0.7f, 0.7f),
        "",
        vec3(1.0f, 1.0f, 1.0f),
        vec3(0.0f, 0.0f, 0.0f),
        0.001f,
        0.0f,
        0.0f,
        0.0f,
        1.0f,
      };
      m_materials[currentMaterial] = m;
    }
    else if(firstword == "diffusereflectance" || firstword == "kd")
    {
      vec3 color;
      ss >> color.x >> color.y >> color.z;
      m_materials[currentMaterial].diffuseReflectance = color;
    }
    else if(firstword == "diffusereflectancemap" || firstword == "map_kd")
    {
      ss >> m_materials[currentMaterial].diffuseReflectanceMap;
    }
    else if(firstword == "specularreflectance" || firstword == "ks")
    {
      vec3 color;
      ss >> color.x >> color.y >> color.z;
      m_materials[currentMaterial].specularReflectance = color;
    }
    else if(firstword == "emittance")
    {
      vec3 color;
      ss >> color.x >> color.y >> color.z;
      m_materials[currentMaterial].emittance = color;
    }
    else if(firstword == "specularroughness")
    {
      ss >> m_materials[currentMaterial].specularRoughness;
    }
    else if(firstword == "transparency")
    {
      ss >> m_materials[currentMaterial].transparency;
    }
    else if(firstword == "reflat0deg")
    {
      ss >> m_materials[currentMaterial].reflAt0Deg;
    }
    else if(firstword == "reflat90deg")
    {
      ss >> m_materials[currentMaterial].reflAt90Deg;
    }
    else if(firstword == "indexofrefraction")
    {
      ss >> m_materials[currentMaterial].indexOfRefraction;
    }
    else if(firstword == "newlight")
    {
      ss >> currentLight;
      Light l =
      {
        vec3(0.7f, 0.7f, 0.7f),
        vec3(1.0f, 1.0f, 1.0f),
        0.2f,
        1.0f
      };
      m_lights[currentLight] = l;
    }
    else if(firstword == "lightposition")
    {
      vec3 pos;
      ss >> pos.x >> pos.y >> pos.z;
      m_lights[currentLight].position = pos;
    }
    else if(firstword == "lightcolor")
    {
      vec3 color;
      ss >> color.x >> color.y >> color.z;
      m_lights[currentLight].color = color;
    }
    else if(firstword == "lightradius")
    {
      ss >> m_lights[currentLight].radius;
    }
    else if(firstword == "lightintensity")
    {
      ss >> m_lights[currentLight].intensity;
    }
    else if(firstword == "newcamera")
    {
      ss >> currentCamera;
      Camera c =
      {
        vec3(0.0f, 0.0f, 10.0f),
        vec3(0.0f, 0.0f, 0.0f),
        vec3(0.0f, 1.0f, 0.0f),
        60.0,
      };
      m_cameras[currentCamera] = c;
    }
    else if(firstword == "cameraposition")
    {
      vec3 pos;
      ss >> pos.x >> pos.y >> pos.z;
      m_cameras[currentCamera].position = pos;
    }
    else if(firstword == "cameratarget")
    {
      vec3 pos;
      ss >> pos.x >> pos.y >> pos.z;
      m_cameras[currentCamera].target = pos;
    }
    else if(firstword == "cameraup")
    {
      vec3 pos;
      ss >> pos.x >> pos.y >> pos.z;
      m_cameras[currentCamera].up = pos;
    }
    else if(firstword == "camerafov")
    {
      ss >> m_cameras[currentCamera].fov;
    }

  }
}

void OBJModel::load(const string& fileName) {
  ifstream file;

  file.open(fileName.c_str(), ios::binary);
  if (!file) {
    stringstream ss;
    ss << "Error in openening file '" << fileName << "'\n";
    throw ss.str();
  }

  string basePath = fileName.substr(0, fileName.find_last_of('/')) + "/";
  loadOBJ(file, basePath);
}
