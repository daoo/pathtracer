#include <experimental/filesystem>
#include <iostream>

#include "util/clock.h"
#include "util/nicetime.h"
#include "wavefront/mtl.h"
#include "wavefront/obj.h"
#include "wavefront/parser.h"

using std::experimental::filesystem::path;

void ObjInfo(path file) {
  util::Clock clock;
  wavefront::Obj obj = wavefront::LoadObj(file);
  double load_time = clock.measure<double, std::ratio<1>>();

  unsigned int triangle_count = 0;
  for (const wavefront::Chunk& c : obj.chunks) {
    triangle_count += c.polygons.size();
  }

  std::cout << "Loaded " << file << " in ";
  std::cout << util::TimeAutoUnit(load_time) << "\n";
  std::cout << "  Chunks:    " << obj.chunks.size() << '\n';
  std::cout << "  Triangles: " << triangle_count << '\n';
}

void MtlInfo(path file) {
  util::Clock clock;
  wavefront::Mtl mtl = wavefront::LoadMtl(file);
  double load_time = clock.measure<double, std::ratio<1>>();

  std::cout << "Loaded " << file << " in ";
  std::cout << util::TimeAutoUnit(load_time) << "\n";
  std::cout << "  Materials: " << mtl.materials.size() << '\n';
}

void WavefrontInfo(path file) {
  if (file.extension() == ".obj") {
    ObjInfo(file);
  } else if (file.extension() == ".mtl") {
    MtlInfo(file);
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
      WavefrontInfo(argv[i]);
    }
  } catch (const wavefront::FileException& ex) {
    std::cerr << ex.what() << '\n';
    return 1;
  }
  return 0;
}
