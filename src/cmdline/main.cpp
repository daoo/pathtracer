#include <iostream>
#include <thread>

#include "pathtracer/pathtracer.hpp"
#include "util/clock.hpp"
#include "util/samplebuffer.hpp"
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
  Pathtracer pt3(w, h, scene, camera);
  Pathtracer pt4(w, h, scene, camera);

  thread t1(trace, ref(pt1), samples / 4);
  thread t2(trace, ref(pt2), samples / 4);
  thread t3(trace, ref(pt3), samples / 4);
  thread t4(trace, ref(pt4), samples / 4);

  t1.join();
  t2.join();
  t3.join();
  t4.join();

  util::SampleBuffer result(w, h);
  result.append(pt1.buffer());
  result.append(pt2.buffer());
  result.append(pt3.buffer());
  result.append(pt4.buffer());

  util::writeImage(output, result);
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
