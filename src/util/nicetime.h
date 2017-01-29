#ifndef UTIL_NICETIME_H_
#define UTIL_NICETIME_H_

#include <iostream>

namespace util {
struct NiceTime {
  unsigned int hour, min, sec;
};

NiceTime nice_time(unsigned int seconds);

std::ostream& operator<<(std::ostream&, const NiceTime&);
}  // namespace util

#endif  // UTIL_NICETIME_H_
