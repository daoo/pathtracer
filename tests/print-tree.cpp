#include "trace/clock.hpp"
#include "trace/kdtree/print.hpp"
#include "trace/scene.hpp"
#include "trace/wavefront/obj.hpp"

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

  const wavefront::Obj obj = wavefront::load_obj(obj_file);
  const wavefront::Mtl mtl = wavefront::load_mtl(obj_file.parent_path() / obj.mtl_lib);

  Clock clock;
  clock.start();
  Scene scene(obj, mtl);
  clock.stop();

  cout << "Built in " << clock.length<float, ratio<1>>() << " seconds.\n\n";
  print(cout, scene.kdtree());

  return 0;
}
