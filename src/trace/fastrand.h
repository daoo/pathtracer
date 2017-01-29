#ifndef TRACE_FASTRAND_H_
#define TRACE_FASTRAND_H_

#include <limits>
#include <random>

namespace trace {
class FastRand {
 public:
  FastRand() : m_engine(std::random_device()()) {}

  float next() {
    return std::generate_canonical<float, std::numeric_limits<float>::digits>(
        m_engine);
  }

 private:
  std::mt19937 m_engine;
};
}  // namespace trace

#endif  // TRACE_FASTRAND_H_
