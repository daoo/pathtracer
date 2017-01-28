#include "util/clock.hpp"
#include "wavefront/mtl.hpp"
#include "wavefront/obj.hpp"

#include <experimental/filesystem>
#include <iostream>

using namespace std::experimental::filesystem;
using namespace std;

int main(int argc, char* argv[]) {
  if (argc <= 1) {
    cerr << "Usage: print-wavefront [PATH]\n";
    return 1;
  }

  try {
    for (int i = 1; i < argc; ++i) {
      path file = argv[i];

      if (file.extension() == ".obj") {
        util::Clock clock;
        clock.start();
        wavefront::Obj obj = wavefront::load_obj(file);
        clock.stop();

        unsigned int triangle_count = 0;
        for (const wavefront::Chunk& c : obj.chunks) {
          triangle_count += c.polygons.size();
        }

        cout << "Loaded " << file << " in ";
        cout << clock.length<float, ratio<1>>() << "seconds\n";
        cout << "  Chunks:    " << obj.chunks.size() << '\n';
        cout << "  Triangles: " << triangle_count << '\n';
      } else if (file.extension() == ".mtl") {
        util::Clock clock;
        clock.start();
        wavefront::Mtl mtl = wavefront::load_mtl(file);
        clock.stop();

        cout << "Loaded " << file << " in ";
        cout << clock.length<float, ratio<1>>() << "seconds\n";
        cout << "  Materials: " << mtl.materials.size() << '\n';
      } else {
        cerr << "Error: " << file << " is not an obj or mtl file.\n";
        return 1;
      }
    }
  } catch (const runtime_error& ex) {
    cerr << ex.what();
    return 1;
  }
  return 0;
}
