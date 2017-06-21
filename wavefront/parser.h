#ifndef WAVEFRONT_PARSE_H_
#define WAVEFRONT_PARSE_H_

#include <glm/glm.hpp>

#include <cassert>
#include <stdexcept>
#include <string>

namespace wavefront {

class Parser {
 public:
  Parser(const char* ptr) : ptr_(ptr) { assert(ptr != nullptr); }

  char ParseChar() {
    char chr = *ptr_;
    ++ptr_;
    return chr;
  }

  std::string ParseString() {
    std::string str(ptr_);
    ptr_ += str.size();
    return str;
  }

  unsigned int ParseUInt() {
    int result = 0;
    const char* num_start = ptr_;
    const char* num_end = ptr_;
    while (*num_end >= '0' && *num_end <= '9') {
      ++num_end;
    }

    const char* ptr = num_end - 1;
    int power = 1;
    while (ptr >= num_start) {
      result += static_cast<int>(*ptr - 48) * power;
      power *= 10;
      --ptr;
    }
    ptr_ = num_end;
    return result;
  }

  int ParseInt() {
    int factor = 1;
    if (*ptr_ == '-') {
      ptr_ += 1;
      factor = -1;
    }
    return factor * ParseUInt();
  }

  float ParseFloat() {
    char* end;
    float x = strtof(ptr_, &end);
    if (ptr_ == end) throw std::runtime_error("invalid float");
    ptr_ = end;
    return x;
  }

  glm::vec2 ParseVec2() {
    float x = ParseFloat();
    float y = ParseFloat();
    return {x, y};
  }

  glm::vec3 ParseVec3() {
    float x = ParseFloat();
    float y = ParseFloat();
    float z = ParseFloat();
    return {x, y, z};
  }

  void Skip(int count) { ptr_ += count; }

  void SkipWhitespace() {
    while (*ptr_ != 0 && (*ptr_ == ' ' || *ptr_ == '\t')) {
      ++ptr_;
    }
  }

  bool Match(const char* str) {
    const char* a = ptr_;
    const char* b = str;
    do {
      if (*a != *b) return false;

      ++a;
      ++b;
    } while (*a != 0 && *b != 0);

    ptr_ = a;

    return true;
  }

  bool AtEnd() const { return *ptr_ == 0; }

 private:
  const char* ptr_;
};

}  // namespace wavefront

#endif  // WAVEFRONT_PARSE_H_
