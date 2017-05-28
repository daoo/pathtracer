#ifndef WAVEFRONT_PARSE_H_
#define WAVEFRONT_PARSE_H_

#include <glm/glm.hpp>

#include <cassert>
#include <string>

namespace wavefront {
inline int parse_int(const char* str, const char** end) {
  int negate = 1;
  if (*str == '-') {
    ++str;
    negate = -1;
  }

  const char* ptr = str;
  while (*ptr >= '0' && *ptr <= '9') {
    ++ptr;
  }

  if (end) *end = ptr;
  --ptr;

  int result = 0;
  int power = 1;
  while (ptr >= str) {
    result += static_cast<int>(*ptr - 48) * power;
    power *= 10;
    --ptr;
  }

  return negate * result;
}

inline bool equal(const char* a, const char* b) {
  assert(a != nullptr);
  assert(*a != 0);
  assert(b != nullptr);

  while (*a != 0) {
    if (*a != *b) return false;

    ++a;
    ++b;
  }

  return true;
}

inline const char* skip_whitespace(const char* str) {
  while (*str != 0 && (*str == ' ' || *str == '\t')) {
    ++str;
  }
  return str;
}

inline const char* next_word(const char* str) {
  // If we point at a word we skip it first
  while (*str != 0 && (*str != ' ' && *str != '\t')) {
    ++str;
  }

  // Then skip any whitespace to the next word
  return skip_whitespace(str);
}

inline std::string parse_string(const char* str) {
  assert(str != nullptr);
  return std::string(str);
}

inline float parse_float(const char* str) {
  assert(str != nullptr);
  char* end;
  float x = strtof(str, &end);
  if (str == end) throw std::runtime_error("invalid float");
  return x;
}

inline glm::vec2 parse_vec2(const char* str) {
  assert(str != nullptr);

  char* endx;
  float x = strtof(str, &endx);
  if (str == endx) throw std::runtime_error("invalid float");

  char* endy;
  float y = strtof(endx, &endy);
  if (endx == endy) throw std::runtime_error("invalid float");

  return {x, y};
}

inline glm::vec3 parse_vec3(const char* str) {
  assert(str != nullptr);

  char* endx;
  float x = strtof(str, &endx);
  if (str == endx) throw std::runtime_error("invalid float");

  char* endy;
  float y = strtof(endx, &endy);
  if (endx == endy) throw std::runtime_error("invalid float");

  char* endz;
  float z = strtof(endy, &endz);
  if (endy == endz) throw std::runtime_error("invalid float");

  return {x, y, z};
}
}  // namespace wavefront

#endif  // WAVEFRONT_PARSE_H_
