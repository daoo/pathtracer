#include "nicetime.hpp"

namespace util
{
  NiceTime niceTime(unsigned int seconds)
  {
    return { seconds / 3600, (seconds % 3600) / 60, seconds % 60 };
  }
}
