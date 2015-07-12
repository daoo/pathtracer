#include "thread.hpp"

#include "trace/clock.hpp"
#include "trace/concurrentqueue.hpp"
#include "trace/fastrand.hpp"
#include "trace/nicetime.hpp"
#include "trace/path.hpp"
#include "trace/pathtracer.hpp"
#include "trace/samplebuffer.hpp"
#include "trace/scene.hpp"
#include "trace/wavefront/mtl.hpp"
#include "trace/wavefront/obj.hpp"

#include <iostream>
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

void print_status(
    unsigned int samples_per_thread,
    const vector<WorkerStatus>& status)
{
  const unsigned int thread_count = status.size();
  const unsigned int samples      = samples_per_thread * thread_count;

  float total_time           = 0;
  unsigned int total_samples = 0;

  for (unsigned int i = 0; i < thread_count; ++i) {
    const WorkerStatus& ws = status[i];

    float avg = ws.total / ws.samples;
    float eta = avg * (samples_per_thread - ws.samples);

    total_samples += ws.samples;
    total_time    += avg;

    cout << "Thread " << i << ": "
      << ws.samples << "/" << samples_per_thread << ", "
      << "avg: " << avg << " sec, eta: "
      << nice_time(static_cast<unsigned int>(eta)) << "\n";
  }

  float avg = total_time / thread_count;

  cout << "Total: "
    << total_samples << "/" << samples << ", "
    << "avg: " << avg << " sec\n\n";
}

void worker(
    const kdtree::KdTreeArray& kdtree,
    const vector<SphereLight>& lights,
    const Pinhole& pinhole,
    unsigned int sample_count,
    unsigned int thread,
    misc::ConcurrentQueue<MessageSample>& queue,
    SampleBuffer& buffer)
{
  assert(sample_count > 0);

  Clock clock;
  FastRand rand;
  while (buffer.samples() < sample_count) {
    clock.start();
    pathtrace(kdtree, lights, pinhole, rand, buffer);
    clock.stop();

    queue.push({thread, buffer.samples(), clock.length<float, ratio<1>>()});
  }
}

void program(
    const path& obj_file,
    const path& out_dir,
    unsigned int width,
    unsigned int height,
    unsigned int camera,
    unsigned int sample_count,
    unsigned int thread_count)
{
  assert(!obj_file.empty());
  assert(width > 0 && height > 0);
  assert(sample_count > 0);
  assert(thread_count > 0);

  // Setup scene
  const wavefront::Obj obj = wavefront::load_obj(obj_file);
  const wavefront::Mtl mtl = wavefront::load_mtl(obj_file.parent_path() / obj.mtl_lib);

  const Scene scene = new_scene(obj, mtl);
  const Pinhole pinhole(scene.cameras[camera], width, height);

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
    threads.emplace_back(
        worker,
        ref(scene.kdtree),
        ref(scene.lights),
        ref(pinhole),
        samples_per_thread,
        i,
        ref(queue),
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

    if (msg.sample == samples_per_thread)
      --working;

    print_status(samples_per_thread, status);
  }

  for (thread& th : threads) {
    th.join();
  }

  // Merge results from each thread
  if (!out_dir.empty()) {
    SampleBuffer result(width, height);
    for (const SampleBuffer& buffer : buffers) {
      result.append(buffer);
    }

    // Make a nice file name and save a file without overwriting anything
    string name = nice_name(obj_file, width, height, sample_count);
    write_image(next_free_name(out_dir, name, ".png").string(), result);
  }
}
