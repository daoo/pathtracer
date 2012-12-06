#include <chrono>
#include <iostream>
#include <sstream>

#include "pathtracer/kdtree/print.hpp"
#include "pathtracer/pathtracer.hpp"
#include "util/image.hpp"
#include "util/strings.hpp"

using namespace std::chrono;
using namespace std;

void program(const string& objFile, const string& output, size_t w, size_t h, size_t camera, size_t samples) {
  OBJModel model;
  model.load(objFile);

  Scene scene(model);

  Pathtracer pt(w, h, scene);
  pt.m_selectedCamera = camera;

  typedef high_resolution_clock clock;
  typedef clock::duration time;

  while (pt.m_frameBufferSamples < samples) {
    time t1 = clock::now().time_since_epoch();
    pt.tracePrimaryRays();
    time t2 = clock::now().time_since_epoch();

    cout << "Sample " << pt.m_frameBufferSamples
         << ", in " << duration_cast<duration<double, ratio<1>>>(t2 - t1).count() << " seconds\n";
  }

  writeImage(output, pt.m_frameBufferWidth, pt.m_frameBufferHeight,
      pt.m_frameBufferSamples, pt.m_frameBuffer);
}

int main(int argc, char* argv[]) {
  if (argc == 6) {
    string obj_file = argv[1];
    string img_file = argv[2];
    size_t width    = parse<size_t>(argv[3]);
    size_t height   = parse<size_t>(argv[4]);
    size_t samples  = parse<size_t>(argv[5]);

    program(argv[0], argv[1], width, height, 0, samples);
  } else {
    cout << "Usage: pathtracer model.obj output.png width height samples\n";
  }

  return 0;
}
