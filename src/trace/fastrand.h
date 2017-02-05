#ifndef TRACE_FASTRAND_H_
#define TRACE_FASTRAND_H_

#include <limits>
#include <random>

namespace trace {
class FastRand {
 public:
  FastRand() : engine_(std::random_device()()) {}

  float next() {
    return std::generate_canonical<float, std::numeric_limits<float>::digits>(
        engine_);
  }

 private:
  std::mt19937 engine_;
};
}  // namespace trace

#endif  // TRACE_FASTRAND_H_
