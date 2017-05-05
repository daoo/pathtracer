#ifndef UTIL_CLOCK_H_
#define UTIL_CLOCK_H_

#include <chrono>

namespace util {
class Clock {
 public:
  Clock() : start_(clock::now().time_since_epoch()) {}

  template <typename T, typename Ratio>
  T measure() {
    clock::duration stop = clock::now().time_since_epoch();
    return std::chrono::duration_cast<std::chrono::duration<T, Ratio>>(stop -
                                                                       start_)
        .count();
  }

 private:
  typedef std::chrono::high_resolution_clock clock;

  clock::duration start_;
};
}  // namespace util

#endif  // UTIL_CLOCK_H_
