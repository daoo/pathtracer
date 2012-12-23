#include "pathtracer/fastrand.hpp"
#include "pathtracer/pathtracer.hpp"
#include "pathtracer/samplebuffer.hpp"
#include "util/clock.hpp"
#include "util/path.hpp"
#include "util/strings.hpp"

#include <boost/filesystem.hpp>
#include <iostream>

using namespace boost;
using namespace boost::filesystem;
using namespace std::chrono;
using namespace std;
using namespace util;

void program(const path& objFile, const path& outDir,
    unsigned int w, unsigned int h, unsigned int camera,
    unsigned int sampleCount)
{
  assert(!objFile.empty());
  assert(!outDir.empty());
  assert(w > 0 && h > 0);
  assert(sampleCount > 0);

  const objloader::Obj obj = objloader::loadObj(objFile);
  const objloader::Mtl mtl = objloader::loadMtl(objFile.parent_path() / obj.mtl_lib);

  const Scene scene(obj, mtl);
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

  string scene_name = basename(change_extension(objFile, ""));
  stringstream name;
  name << scene_name << "_"
        << w << "x" << h << "_"
        << buffer.samples();

  writeImage(nextFreeName(outDir, name.str(), ".png"), buffer);
}

int main(int argc, char* argv[])
{
  if (argc != 6) {
    cerr << "Usage: pathtracer model.obj output-dir width height samples\n";
    return 1;
  }

  string obj_file = argv[1];
  string img_dir  = argv[2];

  unsigned int width   = parse<unsigned int>(argv[3]);
  unsigned int height  = parse<unsigned int>(argv[4]);
  unsigned int samples = parse<unsigned int>(argv[5]);

  program(obj_file, img_dir, width, height, 0, samples);

  return 0;
}
