#include <iostream>
#include <sstream>
#include <thread>

#include "pathtracer/pathtracer.hpp"
#include "util/clock.hpp"
#include "util/image.hpp"
#include "util/strings.hpp"

using namespace std::chrono;
using namespace std;

void trace(Pathtracer& pt, size_t samples) {
  util::Clock clock;
  while (pt.samples() < samples) {
    clock.start();
    pt.tracePrimaryRays();
    clock.stop();

    cout << "Sample " << pt.samples()
         << ", in " << clock.length<float, ratio<1>>() << " seconds\n";
  }
}

void program(const string& objFile, const string& output, size_t w, size_t h, size_t camera, size_t samples) {
  OBJModel model;
  model.load(objFile);

  const Scene scene(model);

  Pathtracer pt1(w, h, scene, camera);
  Pathtracer pt2(w, h, scene, camera);

  thread t1(trace, ref(pt1), samples / 2);
  thread t2(trace, ref(pt2), samples / 2);

  t1.join();
  t2.join();

  Pathtracer pt(merge(pt1, pt2));
  writeImage(output, pt.width(), pt.height(), pt.samples(), pt.buffer());
}

int main(int argc, char* argv[]) {
  if (argc == 6) {
    string obj_file = argv[1];
    string img_file = argv[2];
    size_t width    = parse<size_t>(argv[3]);
    size_t height   = parse<size_t>(argv[4]);
    size_t samples  = parse<size_t>(argv[5]);

    program(obj_file, img_file, width, height, 0, samples);
  } else {
    cout << "Usage: pathtracer model.obj output.png width height samples\n";
  }

  return 0;
}
