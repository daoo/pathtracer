#ifndef UTIL_NICETIME_H_
#define UTIL_NICETIME_H_

#include <iostream>

namespace util {
class TimeSplit {
 public:
  TimeSplit(double seconds)
      : hours_(static_cast<size_t>(seconds / 3600.0)),
        minutes_(static_cast<size_t>((seconds - (hours_ * 3600.0)) / 60.0)),
        seconds_(static_cast<size_t>(seconds - (hours_ * 3600.0) -
                                     (minutes_ * 60.0))) {}

 private:
  friend std::ostream& operator<<(std::ostream&, const TimeSplit&);

  size_t hours_, minutes_, seconds_;
};

class TimeAutoUnit {
 public:
  TimeAutoUnit(double seconds) : seconds_(seconds) {}

 private:
  friend std::ostream& operator<<(std::ostream&, const TimeAutoUnit&);

  double seconds_;
};

std::ostream& operator<<(std::ostream&, const TimeSplit&);
std::ostream& operator<<(std::ostream&, const TimeAutoUnit&);
}  // namespace util

#endif  // UTIL_NICETIME_H_
