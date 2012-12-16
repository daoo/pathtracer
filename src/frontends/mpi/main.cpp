#include "pathtracer/pathtracer.hpp"
#include "pathtracer/samplebuffer.hpp"
#include "pathtracer/scene.hpp"
#include "util/clock.hpp"
#include "util/strings.hpp"

#include <boost/archive/text_iarchive.hpp>
#include <boost/archive/text_oarchive.hpp>
#include <boost/mpi/communicator.hpp>
#include <boost/mpi/environment.hpp>
#include <iostream>

using namespace boost;
using namespace std;
using namespace util;

struct SampleInformation : public mpi::is_mpi_datatype<SampleInformation> {
  SampleInformation() { }
  SampleInformation(size_t count, float time) : count(count), time(time) { }

  size_t count;
  float time;

  template<class Archive>
  void serialize(Archive& ar, const unsigned int)
  {
    ar & count;
    ar & time;
  }
};

enum InitializationMessage { SceneBuilt };
enum ProgressMessage { SamplesCompleted, Finished };

void trace(mpi::communicator, mpi::communicator
    , size_t width, size_t height, size_t sampleCount)
{
  assert(width > 0 && height > 0);
  assert(sampleCount > 0);

  // wait for work
  Scene scene;
  //local.recv(world.rank(), SceneBuilt, scene);

  // start work
  const Pathtracer pt(scene, 0, width, height);

  SampleBuffer buffer(width, height);
  FastRand rand;
  Clock clock;
  while (buffer.samples() < sampleCount) {
    clock.start();
    pt.tracePrimaryRays(rand, buffer);
    clock.stop();

    //world.isend(local.rank(), SamplesCompleted,
        //SampleInformation{buffer.samples(), clock.length<float, ratio<1>>()});
  }

  //world.send(local.rank(), Finished, buffer);
}

int main(int argc, char* argv[])
{
  mpi::environment env(argc, argv);
  mpi::communicator world;

  if (world.size() < 2) {
    cerr << "Needs at least 2 processes to run.";
    return 1;
  }

  if (argc != 6) {
    cerr << "Usage: pathtracer-mpi model.obj output-dir width height samples\n";
    return 1;
  }

  string obj_file = argv[1];
  string img_dir  = argv[2];
  size_t width    = parse<size_t>(argv[3]);
  size_t height   = parse<size_t>(argv[4]);
  size_t samples  = parse<size_t>(argv[5]);

  size_t worker_count = world.size() - 1;
  bool is_worker      = world.rank() == 0;

  mpi::communicator local = world.split(is_worker? 0 : 1);
  if (is_worker) {
    size_t running = worker_count;

    SampleBuffer result(width, height);
    while (running > 0) {
      SampleInformation info;
      world.recv(mpi::any_source, SamplesCompleted, info);

      cout << info.count;
    }
  } else {
    trace(local, world, width, height, samples);
  }

  return 0;
}
