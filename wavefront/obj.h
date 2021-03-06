#ifndef WAVEFRONT_OBJ_H_
#define WAVEFRONT_OBJ_H_

#include <experimental/filesystem>
#include <glm/glm.hpp>
#include <string>
#include <vector>

namespace wavefront {
struct Point {
  int v, t, n;
};
struct Face {
  Point p1, p2, p3;
};

struct Chunk {
  explicit Chunk(const std::string& mtl) : material(mtl) {}

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

Obj LoadObj(const std::experimental::filesystem::path&);

template <typename T>
inline T IndexArray(const std::vector<T>& arr, int index) {
  if (index == 0) {
    return T();
  } else if (index < 0) {
    return arr[static_cast<size_t>(static_cast<int>(arr.size()) + index)];
  } else {
    return arr[static_cast<size_t>(index - 1)];
  }
}

inline glm::vec3 IndexVertex(const Obj& obj, int index) {
  return IndexArray(obj.vertices, index);
}

inline glm::vec3 IndexNormal(const Obj& obj, int index) {
  return IndexArray(obj.normals, index);
}

inline glm::vec2 IndexTexcoord(const Obj& obj, int index) {
  return IndexArray(obj.texcoords, index);
}
}  // namespace wavefront

#endif  // WAVEFRONT_OBJ_H_
