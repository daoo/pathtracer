#ifndef OBJ_HPP_ABWTTNHR
#define OBJ_HPP_ABWTTNHR

#include <boost/filesystem.hpp>

namespace objloader
{
  struct Vertex            { float x, y, z; };
  struct Normal            { float x, y, z; };
  struct TextureCoordinate { float u, v, w; };

  struct Triangle
  {
    int xv, xt, xn;
    int yv, yt, yn;
    int zv, zt, zn;
  };

  struct Material
  {
  };

  struct Chunk
  {
    std::vector<Triangle> triangles;

    std::string material;
  };

  struct Obj
  {
    std::vector<Vertex> vertices;
    std::vector<Normal> normals;
    std::vector<TextureCoordinate> textureCoordinates;

    std::map<std::string, Material> materials;

    std::vector<Chunk> chunks;
  };

  Obj load(const boost::filesystem::path&, const boost::filesystem::path&);
}

#endif /* end of include guard: OBJ_HPP_ABWTTNHR */
