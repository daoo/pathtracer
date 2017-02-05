#include "util/nicetime.h"

#include <iomanip>

using std::ostream;

namespace util {
ostream& operator<<(ostream& stream, const TimeSplit& time) {
  stream << std::setw(2) << std::setfill('0') << time.hours << ":";
  stream << std::setw(2) << std::setfill('0') << time.minutes << ":";
  stream << std::setw(2) << std::setfill('0') << time.seconds;
  return stream;
}
ostream& operator<<(ostream& stream, const TimeAutoUnit& time) {
  if (time.seconds > 3600.0) {
    stream << time.seconds / 3600.0 << "h";
  } else if (time.seconds > 60.0) {
    stream << time.seconds / 60.0 << "m";
  } else if (time.seconds > 1.0) {
    stream << time.seconds << "m";
  } else {
    stream << time.seconds * 1000.0 << "ms";
  }
  return stream;
}
}  // namespace util
