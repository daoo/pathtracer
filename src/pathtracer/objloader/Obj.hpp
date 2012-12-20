#ifndef OBJ_HPP_ABWTTNHR
#define OBJ_HPP_ABWTTNHR

#include <boost/filesystem.hpp>
#include <exception>
#include <map>
#include <ostream>
#include <vector>

/**
 * Obj and Mtl loader.
 *
 * Only supports ASCII.
 */
namespace objloader
{
  class ObjLoaderException : public std::runtime_error
  {
    public:
      ObjLoaderException(const std::string& what) : std::runtime_error(what) { }
  };

  class ObjLoaderParserException : public ObjLoaderException
  {
    public:
      ObjLoaderParserException(const boost::filesystem::path& file,
          size_t line, size_t column, const std::string& text,
          const std::string& message) : ObjLoaderException(message),
          file(file), line(line), column(column), text(text),
          message(message) { }

      boost::filesystem::path file;
      size_t line, column;
      std::string text;

      std::string message;
  };

  std::ostream& operator<<(std::ostream&, const ObjLoaderParserException&);

  struct Vertex   { float x, y, z; };
  struct Normal   { float x, y, z; };
  struct TexCoord { float u, v; };

  struct Triangle
  {
    int v0, t0, n0;
    int v1, t1, n1;
    int v2, t2, n2;
  };

  struct Chunk
  {
    Chunk(const std::string& mtl) : material(mtl) { }

    std::vector<Triangle> triangles;

    std::string material;
  };

  struct Obj
  {
    std::vector<Vertex> vertices;
    std::vector<Normal> normals;
    std::vector<TexCoord> texcoords;

    std::vector<Chunk> chunks;

    boost::filesystem::path mtl_lib;
  };

  struct Material
  {
    float diffuseReflectance[3];
    std::string diffuseReflectanceMap;
    float specularReflectance[3];
    float emittance[3];
    float specularRoughness;
    float transparency;
    float reflAt0Deg;
    float reflAt90Deg;
    float indexOfRefraction;
  };

  struct Mtl
  {
    std::map<std::string, Material> materials;
  };

  Obj loadObj(const boost::filesystem::path&);
  Mtl loadMtl(const boost::filesystem::path&);
}

#endif /* end of include guard: OBJ_HPP_ABWTTNHR */
