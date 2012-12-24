#include "trace/kdtree/print.hpp"
#include "trace/pathtracer.hpp"
#include "util/clock.hpp"

#include <boost/filesystem.hpp>
#include <iostream>

using namespace boost::filesystem;
using namespace std;
using namespace trace;
using namespace util;

int main(int argc, char* argv[])
{
  if (argc != 2) {
    cout << "Usage: print-tree model.obj\n";
    return 1;
  }

  path obj_file = argv[1];

  const obj::Obj obj = obj::loadObj(obj_file);
  const obj::Mtl mtl = obj::loadMtl(obj_file.parent_path() / obj.mtl_lib);

  Clock clock;
  clock.start();
  Scene scene(obj, mtl);
  clock.stop();

  cout << "Built in " << clock.length<double, ratio<1>>() << " seconds.\n\n";
  print(cout, scene.kdtree());

  return 0;
}
