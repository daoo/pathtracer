#include <experimental/filesystem>
#include <iostream>

#include "wavefront/mtl.h"
#include "wavefront/obj.h"
#include "wavefront/parser.h"

using std::experimental::filesystem::path;

void ObjPrint(path file) {
  wavefront::Obj obj = wavefront::LoadObj(file);

  std::cout << "mtllib " << obj.mtl_lib << "\n";

  for (const glm::vec3& v : obj.vertices) {
    std::cout << "v " << v.x << " " << v.y << " " << v.z << "\n";
  }
  for (const glm::vec3& vn : obj.normals) {
    std::cout << "vn " << vn.x << " " << vn.y << " " << vn.z << "\n";
  }
  for (const glm::vec2& vt : obj.texcoords) {
    std::cout << "vt " << vt.x << " " << vt.y << "\n";
  }
  for (const auto& chunk : obj.chunks) {
    std::cout << "usemtl " << chunk.material << "\n";
    for (const wavefront::Face& face : chunk.polygons) {
      std::cout << "f ";
      std::cout << face.p1.v << "//" << face.p1.n << ' ';
      std::cout << face.p2.v << "//" << face.p2.n << ' ';
      std::cout << face.p3.v << "//" << face.p3.n << '\n';
    }
  }
}

void MtlPrint(path file) {
  std::cerr << "Error: not implemented yet\n";
}

void WavefrontPrint(path file) {
  if (file.extension() == ".obj") {
    ObjPrint(file);
  } else if (file.extension() == ".mtl") {
    MtlPrint(file);
  } else {
    std::cerr << "Error: " << file << " is not an obj or mtl file.\n";
  }
}

int main(int argc, char* argv[]) {
  if (argc <= 1) {
    std::cerr << "Usage: " << argv[0] << " [PATH]\n";
    return 1;
  }

  try {
    for (int i = 1; i < argc; ++i) {
      WavefrontPrint(argv[i]);
    }
  } catch (const wavefront::FileException& ex) {
    std::cerr << ex.what() << '\n';
    return 1;
  }
  return 0;
}
