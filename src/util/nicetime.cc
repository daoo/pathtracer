#include "util/nicetime.h"

#include <iomanip>

using std::ostream;

namespace util {
ostream& operator<<(ostream& stream, const TimeSplit& time) {
  stream << std::setw(2) << std::setfill('0') << time.hours_ << ":";
  stream << std::setw(2) << std::setfill('0') << time.minutes_ << ":";
  stream << std::setw(2) << std::setfill('0') << time.seconds_;
  return stream;
}
ostream& operator<<(ostream& stream, const TimeAutoUnit& time) {
  if (time.seconds_ > 3600.0) {
    stream << time.seconds_ / 3600.0 << "h";
  } else if (time.seconds_ > 60.0) {
    stream << time.seconds_ / 60.0 << "m";
  } else if (time.seconds_ > 1.0) {
    stream << time.seconds_ << "m";
  } else {
    stream << time.seconds_ * 1000.0 << "ms";
  }
  return stream;
}
}  // namespace util
