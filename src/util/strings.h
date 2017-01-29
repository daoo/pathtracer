#ifndef UTIL_STRINGS_H_
#define UTIL_STRINGS_H_

#include <sstream>
#include <string>

namespace util {
template <typename T>
T parse(const std::string& str) {
  std::stringstream ss(str);
  T tmp;
  ss >> tmp;
  return tmp;
}
}

#endif  // UTIL_STRINGS_H_
