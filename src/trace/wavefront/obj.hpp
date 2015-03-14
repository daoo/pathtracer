#ifndef OBJ_HPP_BMTCK0HO
#define OBJ_HPP_BMTCK0HO

#include <boost/filesystem.hpp>
#include <glm/glm.hpp>
#include <vector>

namespace trace
{
  namespace wavefront
  {
    struct Point { int v, t, n; };
    struct Face { Point p1, p2, p3; };

    struct Chunk
    {
      Chunk(const std::string& mtl) : material(mtl) { }

      std::vector<Face> polygons;
      std::string material;
    };

    struct Obj
    {
      boost::filesystem::path mtl_lib;

      std::vector<glm::vec3> vertices;
      std::vector<glm::vec3> normals;
      std::vector<glm::vec2> texcoords;
      std::vector<Chunk> chunks;
    };

    Obj load_obj(const boost::filesystem::path&);

    inline glm::vec3 index_vertex(const Obj& obj, int i)
    {
      if (i == 0) return glm::vec3();
      return obj.vertices[i < 0 ? obj.vertices.size() + i : i - 1];
    }

    inline glm::vec3 index_normal(const Obj& obj, int i)
    {
      if (i == 0) return glm::vec3();
      return obj.normals[i < 0 ? obj.normals.size() + i : i - 1];
    }

    inline glm::vec2 index_texcoord(const Obj& obj, int i)
    {
      if (i == 0) return glm::vec2();
      return obj.texcoords[i < 0 ? obj.texcoords.size() + i : i - 1];
    }
  }
}

#endif /* end of include guard: OBJ_HPP_BMTCK0HO */
