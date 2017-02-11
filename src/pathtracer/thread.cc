#include "pathtracer/thread.h"

#include <cassert>
#include <cstddef>
#include <functional>
#include <iomanip>
#include <iostream>
#include <ratio>
#include <thread>
#include <vector>

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

using std::experimental::filesystem::path;

namespace kdtree {
class KdTreeArray;
}

struct MessageSample {
  unsigned int thread;
  unsigned int sample;
  double time;
};

struct WorkerStatus {
  unsigned int samples;
  double total_time;
  double total_squared_time;
};

void print_status(unsigned int total_samples,
                  const std::vector<WorkerStatus>& status) {
  double total_time = 0;
  double total_squared_time = 0;
  unsigned int completed_samples = 0;
  for (const WorkerStatus& ws : status) {
    completed_samples += ws.samples;
    total_time += ws.total_time;
    total_squared_time += ws.total_squared_time;
  }

  double mean_time = total_time / completed_samples;
  double mean_squared_time = total_squared_time / completed_samples;
  double standard_deviation =
      glm::sqrt(mean_squared_time - mean_time * mean_time);
  unsigned int samples_left = total_samples - completed_samples;
  double time_left = samples_left * mean_time / status.size();
  std::cout << "\r[" << completed_samples << "/" << total_samples << "] ";
  std::cout << "mean: " << std::fixed << std::setprecision(1)
            << util::TimeAutoUnit(mean_time) << ", ";
  std::cout << "sdev: " << std::fixed << std::setprecision(1)
            << util::TimeAutoUnit(standard_deviation) << ", ";
  std::cout << "time left: " << util::TimeSplit(time_left);
  std::cout << std::flush;
}

void worker(const kdtree::KdTreeArray& kdtree,
            const std::vector<trace::SphereLight>& lights,
            const trace::Pinhole& pinhole,
            unsigned int sample_count,
            unsigned int thread,
            util::ConcurrentQueue<MessageSample>& queue,
            trace::SampleBuffer& buffer) {
  assert(sample_count > 0);

  trace::FastRand rand;
  while (buffer.samples() < sample_count) {
    util::Clock clock;
    pathtrace(kdtree, lights, pinhole, rand, buffer);
    double trace_time = clock.measure<double, std::ratio<1>>();

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
  trace::Scene scene = trace::new_scene(wavefront::load_obj(obj_file),
                                        wavefront::load_mtl(mtl_file));
  trace::Pinhole pinhole(scene.cameras[camera], width, height);

  // Setup one buffer for each thread
  std::vector<trace::SampleBuffer> buffers;
  for (unsigned int i = 0; i < thread_count; ++i) {
    buffers.emplace_back(width, height);
  }

  // Setup threads and message queue
  util::ConcurrentQueue<MessageSample> queue;
  std::vector<std::thread> threads;
  unsigned int samples_per_thread = sample_count / thread_count;
  for (unsigned int i = 0; i < thread_count; ++i) {
    threads.emplace_back(worker, std::ref(scene.kdtree), std::ref(scene.lights),
                         std::ref(pinhole), samples_per_thread, i,
                         std::ref(queue), std::ref(buffers[i]));
  }

  // Wait for work to finish
  std::vector<WorkerStatus> status(thread_count);
  unsigned int working = thread_count;
  while (working > 0) {
    MessageSample msg;
    queue.wait_and_pop(msg);

    WorkerStatus& ws = status[msg.thread];

    ws.samples = msg.sample;
    ws.total_time += msg.time;
    ws.total_squared_time += msg.time * msg.time;

    if (msg.sample == samples_per_thread) --working;

    print_status(sample_count, status);
  }

  std::cout << '\n';

  for (std::thread& th : threads) {
    th.join();
  }

  // Merge results from each thread
  trace::SampleBuffer result(width, height);
  for (const trace::SampleBuffer& buffer : buffers) {
    result.append(buffer);
  }

  write_image(out_file, result);
}
