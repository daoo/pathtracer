#include "trace/fastrand.hpp"
#include "trace/pathtracer.hpp"
#include "trace/samplebuffer.hpp"
#include "util/clock.hpp"
#include "util/path.hpp"
#include "util/strings.hpp"

#include <boost/filesystem.hpp>
#include <iostream>
#include <thread>

using namespace boost::filesystem;
using namespace boost;
using namespace std::chrono;
using namespace std;
using namespace trace;
using namespace util;

void work(const Pathtracer& pt, unsigned int sampleCount,
   unsigned int thread, SampleBuffer& buffer)
{
  assert(sampleCount > 0);

  Clock clock;
  FastRand rand;
  while (buffer.samples() < sampleCount) {
    clock.start();
    pt.tracePrimaryRays(rand, buffer);
    clock.stop();
    cout << "Thread " << thread << ", sample " << buffer.samples() << " in "
      << clock.length<float,ratio<1>>() << "\n";
  }
}

void program(const path& objFile, const path& outDir,
    unsigned int w, unsigned int h, unsigned int camera,
    unsigned int sampleCount, unsigned int threadCount)
{
  assert(!objFile.empty());
  assert(!outDir.empty());
  assert(w > 0 && h > 0);
  assert(sampleCount > 0);

  const obj::Obj obj = obj::loadObj(objFile);
  const obj::Mtl mtl = obj::loadMtl(objFile.parent_path() / obj.mtl_lib);

  const Pathtracer pt(Scene(obj, mtl), camera, w, h);

  vector<SampleBuffer> buffers;
  for (unsigned int i = 0; i < threadCount; ++i) {
    buffers.emplace_back(w, h);
  }

  vector<thread> threads;
  for (unsigned int i = 0; i < threadCount; ++i) {
    threads.emplace_back(work,
        ref(pt),
        sampleCount / threadCount, i,
        ref(buffers[i]));
  }

  for (thread& th : threads) {
    th.join();
  }

  string scene_name = basename(change_extension(objFile, ""));
  stringstream name;
  name << scene_name << "_"
    << w << "x" << h << "_"
    << sampleCount;

  SampleBuffer result(w, h);
  for (const SampleBuffer& buffer : buffers) {
    result.append(buffer);
  }
  writeImage(nextFreeName(outDir, name.str(), ".png"), result);
}

int main(int argc, char* argv[])
{
  if (argc != 7) {
    cerr << "Usage: pathtracer model.obj output-dir width height samples threads\n";
    return 1;
  }

  string obj_file = argv[1];
  string img_dir  = argv[2];

  unsigned int width   = parse<unsigned int>(argv[3]);
  unsigned int height  = parse<unsigned int>(argv[4]);
  unsigned int samples = parse<unsigned int>(argv[5]);
  unsigned int threads = parse<unsigned int>(argv[6]);

  try {
    program(obj_file, img_dir, width, height, 0, samples, threads);
  } catch (const string& str) {
    cerr << str;
  }

  return 0;
}
