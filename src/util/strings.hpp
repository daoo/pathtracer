#ifndef STRINGS_HPP_EZTZBDSY
#define STRINGS_HPP_EZTZBDSY

#include <string>
#include <sstream>

namespace util
{
  template <typename T>
  T parse(const std::string& str)
  {
    std::stringstream ss(str);
    T tmp;
    ss >> tmp;
    return tmp;
  }
}

#endif /* end of include guard: STRINGS_HPP_EZTZBDSY */
