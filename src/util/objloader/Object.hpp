#ifndef OBJECT_HPP_VQO9VLPJ
#define OBJECT_HPP_VQO9VLPJ

namespace util {
  namespace objloader {

    struct Vertex            { float x, y, z; };
    struct Normal            { float x, y, z; };
    struct TextureCoordinate { float u, v, w; };

    struct Triangle {
      int xv, xt, xn;
      int yv, yt, yn;
      int zv, zt, zn;
    };

    struct Chunk {
      std::vector<Triangle> triangles;

      std::string material;
    };

    struct Obj {
      std::vector<Vertex> vertices;
      std::vector<Normal> normals;
      std::vector<TextureCoordinate> textureCoordinates;

      std::vector<Chunk> chunks;
    };
  }
}

#endif /* end of include guard: OBJECT_HPP_VQO9VLPJ */
