#include "util/clock.hpp"
#include "wavefront/mtl.hpp"
#include "wavefront/obj.hpp"

#include <experimental/filesystem>
#include <iostream>

using namespace std::experimental::filesystem;
using namespace std;

int main(int argc, char* argv[]) {
  if (argc != 3) {
    cout << "Usage: print-obj model.obj materials.mtl\n";
    return 1;
  }

  path obj_file = argv[1];
  path mtl_file = argv[2];

  try {
    util::Clock c1;
    c1.start();
    wavefront::Obj obj = wavefront::load_obj(obj_file);
    c1.stop();

    util::Clock c2;
    c2.start();
    wavefront::Mtl mtl = wavefront::load_mtl(mtl_file);
    c2.stop();

    unsigned int triangle_count = 0;
    for (const wavefront::Chunk& c : obj.chunks) {
      triangle_count += c.polygons.size();
    }

    cout << "Loaded " << obj_file << " in ";
    cout << c1.length<float, ratio<1>>() << "seconds\n";
    cout << "  Chunks:    " << obj.chunks.size() << '\n';
    cout << "  Triangles: " << triangle_count << '\n';

    cout << "Loaded " << mtl_file << " in ";
    cout << c2.length<float, ratio<1>>() << "seconds\n";
    cout << "  Materials: " << mtl.materials.size() << '\n';

    return 0;
  } catch (const runtime_error& ex) {
    cerr << ex.what();
    return 1;
  }
}
