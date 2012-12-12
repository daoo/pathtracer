#include <iostream>
#include <thread>

#include "pathtracer/pathtracer.hpp"
#include "util/clock.hpp"
#include "util/fastrand.hpp"
#include "util/samplebuffer.hpp"
#include "util/strings.hpp"

using namespace std::chrono;
using namespace std;
using namespace util;

void trace(Pathtracer& pt, SampleBuffer& buffer, size_t samples)
{
  assert(samples > 0);
  FastRand rand;

  Clock clock;
  while (buffer.samples() < samples) {
    clock.start();
    pt.tracePrimaryRays(rand, buffer);
    clock.stop();

    cout << "Sample " << buffer.samples()
         << ", in " << clock.length<float, ratio<1>>() << " seconds\n";
  }
}

void program(const string& objFile, const string& outFile, size_t w, size_t h,
    size_t camera, size_t sampleCount, size_t threadCount)
{
  assert(!objFile.empty());
  assert(!outFile.empty());
  assert(w > 0 && h > 0);
  assert(threadCount > 0);
  assert(sampleCount > 0);

  OBJModel model;
  model.load(objFile);

  const Scene scene(model);

  size_t samples_thread = sampleCount / threadCount;

  Pathtracer pt(scene, camera, w, h);

  std::vector<SampleBuffer> buffers;
  for (size_t i = 0; i < threadCount; ++i) {
    buffers.push_back(SampleBuffer(w, h));
  }

  std::vector<thread> threads;
  for (SampleBuffer& buffer : buffers) {
    threads.push_back(thread(trace, ref(pt), ref(buffer), samples_thread));
  }

  for (thread& th : threads) {
    th.join();
  }

  SampleBuffer result(w, h);
  for (const SampleBuffer& buffer : buffers) {
    result.append(buffer);
  }
  writeImage(outFile, result);
}

int main(int argc, char* argv[])
{
  if (argc == 7) {
    string obj_file = argv[1];
    string img_file = argv[2];
    size_t width    = parse<size_t>(argv[3]);
    size_t height   = parse<size_t>(argv[4]);
    size_t samples  = parse<size_t>(argv[5]);
    size_t threads  = parse<size_t>(argv[6]);

    program(obj_file, img_file, width, height, 0, samples, threads);
  } else {
    cerr << "Usage: pathtracer model.obj output.png width height samples threads\n";
  }

  return 0;
}
