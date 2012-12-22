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

  struct Vec2
  {
    float x, y;
  };

  struct Vec3
  {
    float x, y, z;
  };

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
    std::vector<Vec3> vertices;
    std::vector<Vec3> normals;
    std::vector<Vec2> texcoords;

    std::vector<Chunk> chunks;

    boost::filesystem::path mtl_lib;
  };

  struct Material
  {
    std::string name;

    std::string diffuseReflectanceMap;
    Vec3 diffuseReflectance, specularReflectance, emittance;
    float specularRoughness;
    float transparency;
    float reflAt0Deg, reflAt90Deg;
    float ior;
  };

  struct Light
  {
    Vec3 position, color;
    float radius, intensity;
  };

  struct Camera
  {
    Vec3 position, target, up;
    float fov;
  };

  struct Mtl
  {
    std::vector<Material> materials;
    std::vector<Light> lights;
    std::vector<Camera> cameras;
  };

  Obj loadObj(const boost::filesystem::path&);
  Mtl loadMtl(const boost::filesystem::path&);
}

#endif /* end of include guard: OBJ_HPP_ABWTTNHR */
