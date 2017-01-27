#ifndef NICETIME_HPP_MTXCTALS
#define NICETIME_HPP_MTXCTALS

#include <iostream>

namespace util
{
  struct NiceTime
  {
    unsigned int hour, min, sec;
  };

  NiceTime nice_time(unsigned int seconds);

  std::ostream& operator<<(std::ostream&, const NiceTime&);
}

#endif /* end of include guard: NICETIME_HPP_MTXCTALS */

