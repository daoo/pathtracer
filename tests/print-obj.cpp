#include "pathtracer/objloader/Obj.hpp"
#include "util/clock.hpp"

#include <boost/filesystem.hpp>
#include <iostream>

using namespace boost::filesystem;
using namespace objloader;
using namespace std;
using namespace util;

int main(int argc, char* argv[])
{
  if (argc != 2) {
    cout << "Usage: print-obj model.obj\n";
    return 1;
  }

  path file = argv[1];

  try {
    Clock clock;
    clock.start();
    Obj obj = loadObj(file);
    clock.stop();

    size_t triangle_count = 0;
    for (const Chunk& c : obj.m_chunks) {
      triangle_count += c.m_triangles.size();
    }

    cout << "Loaded " << file << " in " << clock.length<double, ratio<1>>() << "seconds\n";
    cout << "  Triangles: " << triangle_count << "\n";
    cout << "  Chunks:    " << obj.m_chunks.size() << "\n";

    return 0;
  } catch (const ObjLoaderParserException& ex) {
    cerr << ex;
    return 1;
  }
}
