#include "pathtracer/objloader.hpp"
#include "util/clock.hpp"

#include <boost/filesystem.hpp>
#include <iostream>

using namespace boost::filesystem;
using namespace objloader;
using namespace std;
using namespace util;

int main(int argc, char* argv[])
{
  if (argc != 3) {
    cout << "Usage: print-obj model.obj materials.mtl\n";
    return 1;
  }

  path obj_file = argv[1];
  path mtl_file = argv[2];

  try {
    Clock c1;
    c1.start();
    Obj obj = loadObj(obj_file);
    c1.stop();

    Clock c2;
    c2.start();
    Mtl mtl = loadMtl(mtl_file);
    c2.stop();

    unsigned int triangle_count = 0;
    for (const Chunk& c : obj.chunks) {
      triangle_count += c.triangles.size();
    }

    cout << "Loaded " << obj_file << " in ";
    cout << c1.length<double, ratio<1>>() << "seconds\n";
    cout << "  Chunks:    " << obj.chunks.size() << '\n';
    cout << "  Triangles: " << triangle_count << '\n';

    cout << "Loaded " << mtl_file << " in ";
    cout << c2.length<double, ratio<1>>() << "seconds\n";
    cout << "  Materials: " << mtl.materials.size() << '\n';

    return 0;
  } catch (const runtime_error& ex) {
    cerr << ex.what();
    return 1;
  }
}
