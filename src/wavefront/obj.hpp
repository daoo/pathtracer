#ifndef OBJ_HPP_BMTCK0HO
#define OBJ_HPP_BMTCK0HO

#include <experimental/filesystem>
#include <glm/glm.hpp>
#include <vector>

namespace trace {
namespace wavefront {
struct Point {
  int v, t, n;
};
struct Face {
  Point p1, p2, p3;
};

struct Chunk {
  Chunk(const std::string& mtl) : material(mtl) {}

  std::vector<Face> polygons;
  std::string material;
};

struct Obj {
  std::experimental::filesystem::path mtl_lib;

  std::vector<glm::vec3> vertices;
  std::vector<glm::vec3> normals;
  std::vector<glm::vec2> texcoords;
  std::vector<Chunk> chunks;
};

Obj load_obj(const std::experimental::filesystem::path&);

template <typename T>
inline T index_array(const std::vector<T>& arr, int index) {
  if (index == 0) {
    return T();
  } else if (index < 0) {
    return arr[static_cast<size_t>(static_cast<int>(arr.size()) + index)];
  } else {
    return arr[static_cast<size_t>(index - 1)];
  }
}

inline glm::vec3 index_vertex(const Obj& obj, int index) {
  return index_array(obj.vertices, index);
}

inline glm::vec3 index_normal(const Obj& obj, int index) {
  return index_array(obj.normals, index);
}

inline glm::vec2 index_texcoord(const Obj& obj, int index) {
  return index_array(obj.texcoords, index);
}
}
}

#endif /* end of include guard: OBJ_HPP_BMTCK0HO */
