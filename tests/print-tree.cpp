#include <iostream>

#include "pathtracer/kdtree/print.hpp"
#include "pathtracer/pathtracer.hpp"
#include "util/clock.hpp"

using namespace std;
using namespace util;

int main(int argc, char* argv[])
{
  if (argc != 2) {
    cout << "Usage: print-tree model.obj\n";
    return 1;
  }

  typedef high_resolution_clock clock;
  typedef clock::duration time;

  string obj_file = argv[1];

  Clock clock;
  OBJModel model;

  clock.start();
  model.load(obj_file);
  Scene scene(model);
  clock.stop();

  cout << "Built in " << clock.length<double, ratio<1>>() << " seconds.\n\n";
  print(cout, scene.kdtree());

  return 0;
}
