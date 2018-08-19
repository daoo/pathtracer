#ifndef UTIL_VECTOR_H_
#define UTIL_VECTOR_H_

#include <vector>

namespace util {
template <class T>
void append(std::vector<T>* a, const std::vector<T>& b) {
  a->insert(a->end(), b.cbegin(), b.cend());
}
}  // namespace util

#endif  // UTIL_VECTOR_H_
