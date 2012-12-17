#include <chrono>
#include <iostream>

#include "pathtracer/kdtree/print.hpp"
#include "pathtracer/pathtracer.hpp"

using namespace std::chrono;
using namespace std;

int main(int argc, char* argv[])
{
  if (argc != 2) {
    cout << "Usage: print-tree model.obj\n";
    return 1;
  }

  typedef high_resolution_clock clock;
  typedef clock::duration time;

  string obj_file = argv[1];

  time t1 = clock::now().time_since_epoch();

  OBJModel model;
  model.load(obj_file);
  Scene scene(model);

  time t2 = clock::now().time_since_epoch();

  cout << "Built in " << duration_cast<duration<double, ratio<1>>>(t2 - t1).count() << " seconds.\n\n";
  print(cout, scene.kdtree());

  return 0;
}
