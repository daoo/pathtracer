#ifndef CLOCK_HPP_JEKW7LSY
#define CLOCK_HPP_JEKW7LSY

#include <chrono>

namespace util {
  class Clock {
    public:
      Clock() { }
      ~Clock() { }

      void start() {
        m_start = clock::now().time_since_epoch();
      }

      void stop() {
        m_stop = clock::now().time_since_epoch();
      }

      template <typename T, typename Ratio>
      T length() {
        return std::chrono::duration_cast<std::chrono::duration<T, Ratio>>(m_stop - m_start).count();
      }

    private:
      typedef std::chrono::high_resolution_clock clock;
      typedef clock::duration time;

      time m_start, m_stop;
  };
}

#endif /* end of include guard: CLOCK_HPP_JEKW7LSY */
