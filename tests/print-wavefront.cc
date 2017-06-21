#include <experimental/filesystem>
#include <iostream>

#include "util/clock.h"
#include "wavefront/mtl.h"
#include "wavefront/obj.h"

using std::experimental::filesystem::path;

int main(int argc, char* argv[]) {
  if (argc <= 1) {
    std::cerr << "Usage: print-wavefront [PATH]\n";
    return 1;
  }

  try {
    for (int i = 1; i < argc; ++i) {
      path file = argv[i];

      if (file.extension() == ".obj") {
        util::Clock clock;
        wavefront::Obj obj = wavefront::LoadObj(file);
        float load_time = clock.measure<float, std::ratio<1>>();

        unsigned int triangle_count = 0;
        for (const wavefront::Chunk& c : obj.chunks) {
          triangle_count += c.polygons.size();
        }

        std::cout << "Loaded " << file << " in ";
        std::cout << load_time << " second(s)\n";
        std::cout << "  Chunks:    " << obj.chunks.size() << '\n';
        std::cout << "  Triangles: " << triangle_count << '\n';
      } else if (file.extension() == ".mtl") {
        util::Clock clock;
        wavefront::Mtl mtl = wavefront::LoadMtl(file);
        float load_time = clock.measure<float, std::ratio<1>>();

        std::cout << "Loaded " << file << " in ";
        std::cout << load_time << " second(s)\n";
        std::cout << "  Materials: " << mtl.materials.size() << '\n';
      } else {
        std::cerr << "Error: " << file << " is not an obj or mtl file.\n";
        return 1;
      }
    }
  } catch (const std::runtime_error& ex) {
    std::cerr << ex.what();
    return 1;
  }
  return 0;
}
