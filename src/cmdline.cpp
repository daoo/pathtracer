#include <iostream>

#include "pt/pathtracer.hpp"
#include "util/misc.hpp"

using namespace std;

#ifndef NDEBUG
constexpr int MAX_SAMPLES_PER_PIXEL = 2048;
#else
constexpr int MAX_SAMPLES_PER_PIXEL = 2;
#endif

int main(int, char**) {
  Pathtracer pt(512, 512);

  OBJModel model;

  model.load("scenes/cornell.obj");
  //model.load("scenes/cornell_textured.obj");
  //model.load("scenes/cornellbottle2.obj");

  pt.m_scene = new Scene;
  pt.m_scene->buildFromObj(&model);
  pt.m_selectedCamera = 0;

  //while (pt.m_frameBufferSamples < MAX_SAMPLES_PER_PIXEL) {
    pt.tracePrimaryRays();
  //}

  writeImage("image.png", pt.m_frameBufferSamples,
      pt.m_frameBufferWidth, pt.m_frameBufferHeight,
      pt.m_frameBuffer);

  return 0;
}
