#include "pathtracer/pathtracer.hpp"
#include "util/clock.hpp"
#include "util/fastrand.hpp"
#include "util/samplebuffer.hpp"
#include "util/strings.hpp"

#include <boost/mpi/communicator.hpp>
#include <boost/mpi/environment.hpp>
#include <iostream>
#include <thread>

using namespace boost;
using namespace std::chrono;
using namespace std;
using namespace util;

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
  const Pathtracer pt(scene, camera, w, h);
  SampleBuffer buffer(w, h);
  FastRand rand;

  Clock clock;
  while (buffer.samples() < sampleCount) {
    clock.start();
    pt.tracePrimaryRays(rand, buffer);
    clock.stop();

    cout << "Sample " << buffer.samples()
         << ", in " << clock.length<float, ratio<1>>() << " seconds\n";
  }

  writeImage(outFile, buffer);
}

int main(int argc, char* argv[])
{
  mpi::environment(argc, argv);
  mpi::communicator world;
  cout << "World rank: " << world.rank() << ", size: " << world.size() << endl;

  return 0;
}
