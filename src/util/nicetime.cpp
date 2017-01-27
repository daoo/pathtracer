#include "nicetime.hpp"

using namespace std;

namespace util {
NiceTime nice_time(unsigned int seconds) {
  return {seconds / 3600, (seconds % 3600) / 60, seconds % 60};
}

ostream& operator<<(ostream& stream, const NiceTime& time) {
  stream << time.hour << " h, " << time.min << " min, " << time.sec << " sec";
  return stream;
}
}
