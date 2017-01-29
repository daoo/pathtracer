#ifndef UTIL_CLOCK_H_
#define UTIL_CLOCK_H_

#include <chrono>

namespace util {
class Clock {
 public:
  Clock() : m_start(clock::now().time_since_epoch()) {}

  template <typename T, typename Ratio>
  T measure() {
    clock::duration stop = clock::now().time_since_epoch();
    return std::chrono::duration_cast<std::chrono::duration<T, Ratio>>(stop -
                                                                       m_start)
        .count();
  }

 private:
  typedef std::chrono::high_resolution_clock clock;

  clock::duration m_start;
};
}  // namespace util

#endif  // UTIL_CLOCK_H_
