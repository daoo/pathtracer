#ifndef TRACE_FASTRAND_H_
#define TRACE_FASTRAND_H_

#include <limits>
#include <random>

namespace trace {
class FastRand {
 public:
  FastRand() : engine_(std::random_device()()) {}

  float unit() {
    return std::generate_canonical<float, std::numeric_limits<float>::digits>(
        engine_);
  }

  float range(float a, float b) {
    std::uniform_real_distribution<> distribution(static_cast<double>(a),
                                                  static_cast<double>(b));
    return static_cast<float>(distribution(engine_));
  }

 private:
  std::mt19937 engine_;
};
}  // namespace trace

#endif  // TRACE_FASTRAND_H_
