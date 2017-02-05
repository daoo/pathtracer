#ifndef UTIL_NICETIME_H_
#define UTIL_NICETIME_H_

#include <iostream>

namespace util {
struct TimeSplit {
  TimeSplit(double total_seconds)
      : hours(total_seconds / 3600),
        minutes((total_seconds - (hours * 3600)) / 60),
        seconds(total_seconds - (hours * 3600) - (minutes * 60)) {}

  size_t hours, minutes, seconds;
};

struct TimeAutoUnit {
  TimeAutoUnit(double seconds) : seconds(seconds) {}
  double seconds;
};

std::ostream& operator<<(std::ostream&, const TimeSplit&);
std::ostream& operator<<(std::ostream&, const TimeAutoUnit&);
}  // namespace util

#endif  // UTIL_NICETIME_H_
