#include "thread.h"

#include "trace/camera.h"
#include "trace/fastrand.h"
#include "trace/light.h"
#include "trace/pathtracer.h"
#include "trace/samplebuffer.h"
#include "trace/scene.h"
#include "util/clock.h"
#include "util/concurrentqueue.h"
#include "util/nicetime.h"
#include "wavefront/mtl.h"
#include "wavefront/obj.h"
#include <cassert>
#include <cstddef>
#include <functional>
#include <ratio>
#include <vector>

#include <iostream>
#include <thread>

using namespace std::experimental::filesystem;
using namespace std;
using namespace trace;
using namespace util;

namespace kdtree {
class KdTreeArray;
}

struct MessageSample {
  unsigned int thread;
  unsigned int sample;
  float time;
};

struct WorkerStatus {
  unsigned int samples;
  float total;
};

void print_status(unsigned int samples_per_thread,
                  const vector<WorkerStatus>& status) {
  size_t thread_count = status.size();
  size_t samples = samples_per_thread * thread_count;

  float total_time = 0;
  unsigned int total_samples = 0;

  for (unsigned int i = 0; i < thread_count; ++i) {
    const WorkerStatus& ws = status[i];

    float avg = ws.total / ws.samples;
    float eta = avg * (samples_per_thread - ws.samples);

    total_samples += ws.samples;
    total_time += avg;

    cout << "Thread " << i << ": " << ws.samples << "/" << samples_per_thread
         << ", "
         << "avg: " << avg
         << " sec, eta: " << nice_time(static_cast<unsigned int>(eta)) << "\n";
  }

  float avg = total_time / thread_count;

  cout << "Total: " << total_samples << "/" << samples << ", "
       << "avg: " << avg << " sec\n\n";
}

void worker(const kdtree::KdTreeArray& kdtree,
            const vector<SphereLight>& lights,
            const Pinhole& pinhole,
            unsigned int sample_count,
            unsigned int thread,
            misc::ConcurrentQueue<MessageSample>& queue,
            SampleBuffer& buffer) {
  assert(sample_count > 0);

  FastRand rand;
  while (buffer.samples() < sample_count) {
    Clock clock;
    pathtrace(kdtree, lights, pinhole, rand, buffer);
    float trace_time = clock.measure<float, ratio<1>>();

    queue.push({thread, buffer.samples(), trace_time});
  }
}

void program(const path& obj_file,
             const path& mtl_file,
             const path& out_file,
             unsigned int width,
             unsigned int height,
             unsigned int camera,
             unsigned int sample_count,
             unsigned int thread_count) {
  assert(!obj_file.empty());
  assert(width > 0 && height > 0);
  assert(sample_count > 0);
  assert(thread_count > 0);

  // Setup scene
  Scene scene =
      new_scene(wavefront::load_obj(obj_file), wavefront::load_mtl(mtl_file));
  Pinhole pinhole(scene.cameras[camera], width, height);

  // Setup one buffer for each thread
  vector<SampleBuffer> buffers;
  for (unsigned int i = 0; i < thread_count; ++i) {
    buffers.emplace_back(width, height);
  }

  // Setup threads and message queue
  misc::ConcurrentQueue<MessageSample> queue;
  vector<thread> threads;
  unsigned int samples_per_thread = sample_count / thread_count;
  for (unsigned int i = 0; i < thread_count; ++i) {
    threads.emplace_back(worker, ref(scene.kdtree), ref(scene.lights),
                         ref(pinhole), samples_per_thread, i, ref(queue),
                         ref(buffers[i]));
  }

  // Wait for work to finish
  vector<WorkerStatus> status(thread_count);
  unsigned int working = thread_count;
  while (working > 0) {
    MessageSample msg;
    queue.wait_and_pop(msg);

    WorkerStatus& ws = status[msg.thread];

    ws.samples = msg.sample;
    ws.total += msg.time;

    if (msg.sample == samples_per_thread) --working;

    print_status(samples_per_thread, status);
  }

  for (thread& th : threads) {
    th.join();
  }

  // Merge results from each thread
  SampleBuffer result(width, height);
  for (const SampleBuffer& buffer : buffers) {
    result.append(buffer);
  }

  write_image(out_file, result);
}
