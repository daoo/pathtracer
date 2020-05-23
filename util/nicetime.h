#ifndef UTIL_NICETIME_H_
#define UTIL_NICETIME_H_

#include <iostream>

namespace util {
class TimeSplit {
 public:
  explicit TimeSplit(size_t seconds)
      : hours_(seconds / 3600),
        minutes_((seconds - (hours_ * 3600)) / 60),
        seconds_(seconds - (hours_ * 3600) - (minutes_ * 60)) {}

 private:
  friend std::ostream& operator<<(std::ostream&, const TimeSplit&);

  size_t hours_, minutes_, seconds_;
};

class TimeAutoUnit {
 public:
  explicit TimeAutoUnit(double seconds) : seconds_(seconds) {}

 private:
  friend std::ostream& operator<<(std::ostream&, const TimeAutoUnit&);

  double seconds_;
};

std::ostream& operator<<(std::ostream&, const TimeSplit&);
std::ostream& operator<<(std::ostream&, const TimeAutoUnit&);
}  // namespace util

#endif  // UTIL_NICETIME_H_
