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
          m_file(file), m_line(line), m_column(column), m_text(text),
          m_message(message) { }

      boost::filesystem::path m_file;
      size_t m_line, m_column;
      std::string m_text;

      std::string m_message;
  };

  std::ostream& operator<<(std::ostream&, const ObjLoaderParserException&);

  struct Vertex   { float x, y, z; };
  struct Normal   { float x, y, z; };
  struct TexCoord { float u, v; };

  struct Triangle
  {
    int xv, xt, xn;
    int yv, yt, yn;
    int zv, zt, zn;
  };

  struct Chunk
  {
    std::vector<Triangle> m_triangles;

    std::string m_material;
  };

  struct Obj
  {
    std::vector<Vertex> m_vertices;
    std::vector<Normal> m_normals;
    std::vector<TexCoord> m_texcoords;

    std::vector<Chunk> m_chunks;

    boost::filesystem::path m_mtl_lib;
  };

  struct Material
  {
  };

  struct Mtl
  {
    std::map<std::string, Material> m_materials;
  };

  Obj loadObj(const boost::filesystem::path&);
  Mtl loadMtl(const boost::filesystem::path&);
}

#endif /* end of include guard: OBJ_HPP_ABWTTNHR */
