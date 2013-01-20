#include "thread.hpp"

#include "trace/fastrand.hpp"
#include "trace/pathtracer.hpp"
#include "trace/samplebuffer.hpp"
#include "util/clock.hpp"
#include "util/concurrentqueue.hpp"
#include "util/path.hpp"

#include <iostream>
#include <sstream>
#include <thread>

using namespace boost::filesystem;
using namespace std;
using namespace trace;
using namespace util;

struct MessageSample
{
  unsigned int thread;
  unsigned int sample;
  float time;
};

struct WorkerStatus
{
  unsigned int samples;
  float total;
};

void nice_time(float s, float& hour, float& min, float& sec)
{
  float a = s / 3600.0f;
  hour = floor(a);
  float b = (a - hour) * 60.0f;
  min = floor(b);
  sec = round((b - min) * 60.0f);
}

void print_status(unsigned int samplesPerThread, const vector<WorkerStatus>& status)
{
  system("clear");

  for (unsigned int i = 0; i < status.size(); ++i) {
    const WorkerStatus& ws = status[i];
    float avg = ws.total / ws.samples;

    float eta = avg * (samplesPerThread - ws.samples);
    float h, m, s;
    nice_time(eta, h, m, s);

    cout << "Thread " << i << ": "
      << ws.samples << "/" << samplesPerThread << ", "
      << "avg: " << avg << " sec, eta: "
      << h << " h, " << m << " min, " << s << " sec\n";
  }
}

void worker(const Pathtracer& pt, unsigned int sampleCount,
   unsigned int thread, misc::concurrent_queue<MessageSample>& queue,
   SampleBuffer& buffer)
{
  assert(sampleCount > 0);

  Clock clock;
  FastRand rand;
  while (buffer.samples() < sampleCount) {
    clock.start();
    pt.tracePrimaryRays(rand, buffer);
    clock.stop();

    queue.push({thread, buffer.samples(), clock.length<float, ratio<1>>()});
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
  assert(threadCount > 0);

  // Setup scene
  const obj::Obj obj = obj::loadObj(objFile);
  const obj::Mtl mtl = obj::loadMtl(objFile.parent_path() / obj.mtl_lib);

  const Scene scene(obj, mtl);
  const Pathtracer pt(scene, camera, w, h);

  // Setup one buffer for each thread
  vector<SampleBuffer> buffers;
  for (unsigned int i = 0; i < threadCount; ++i) {
    buffers.emplace_back(w, h);
  }

  // Setup threads and message queue
  misc::concurrent_queue<MessageSample> queue;
  vector<thread> threads;

  unsigned int samplesPerThread = sampleCount / threadCount;
  for (unsigned int i = 0; i < threadCount; ++i) {
    threads.emplace_back(worker,
        ref(pt), samplesPerThread, i, ref(queue), ref(buffers[i]));
  }

  // Wait for work to finish
  vector<WorkerStatus> status(threadCount);
  unsigned int working = threadCount;
  unsigned int messagesRecieved = 0;
  while (working > 0) {
    MessageSample msg;
    queue.wait_and_pop(msg);

    WorkerStatus& ws = status[msg.thread];

    ws.samples = msg.sample;
    ws.total += msg.time;

    if (msg.sample == samplesPerThread)
      --working;

    if (messagesRecieved > threadCount) {
      print_status(samplesPerThread, status);
      messagesRecieved = 0;
    }

    ++messagesRecieved;
  }

  for (thread& th : threads) {
    th.join();
  }

  // Merge results from each thread
  SampleBuffer result(w, h);
  for (const SampleBuffer& buffer : buffers) {
    result.append(buffer);
  }

  // Make a nice file name and save a file without overwriting anything
  string scene_name = basename(change_extension(objFile, ""));
  stringstream name;
  name << scene_name << "_"
    << w << "x" << h << "_"
    << sampleCount;
  writeImage(nextFreeName(outDir, name.str(), ".png"), result);
}
