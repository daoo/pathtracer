#include <iostream>
#include <chrono>

#include "tracer/pathtracer.hpp"
#include "util/image.hpp"

using namespace std::chrono;
using namespace std;

constexpr size_t MAX_SAMPLES_PER_PIXEL = 2048;

int main(int, char**) {
  OBJModel model;
  model.load("scenes/cornell.obj");
  //model.load("scenes/cornell_textured.obj");
  //model.load("scenes/cornellbottle2.obj");

  Scene scene;
  scene.buildFromObj(&model);

  Pathtracer pt(512, 512, scene);
  pt.m_selectedCamera = 0;

  typedef high_resolution_clock clock;
  typedef clock::duration time;

  while (pt.m_frameBufferSamples < MAX_SAMPLES_PER_PIXEL) {
    time t1 = clock::now().time_since_epoch();
    pt.tracePrimaryRays();
    time t2 = clock::now().time_since_epoch();

    cout << "Sample " << pt.m_frameBufferSamples
         << ", in " << duration_cast<duration<double, ratio<1>>>(t2 - t1).count() << " seconds\n";
  }

  writeImage("image.png", pt.m_frameBufferWidth, pt.m_frameBufferHeight,
      pt.m_frameBufferSamples, pt.m_frameBuffer);

  return 0;
}
