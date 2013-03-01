#ifndef NICETIME_HPP_MTXCTALS
#define NICETIME_HPP_MTXCTALS

namespace util
{
  struct NiceTime
  {
    unsigned int hour, min, sec;
  };

  NiceTime niceTime(unsigned int seconds);
}

#endif /* end of include guard: NICETIME_HPP_MTXCTALS */

