#ifndef WAVEFRONT_PARSER_H_
#define WAVEFRONT_PARSER_H_

#include <experimental/filesystem>
#include <glm/glm.hpp>

#include <cassert>
#include <string>

namespace wavefront {

class StringException {
 public:
  StringException(const std::string& str,
                  const char* ptr,
                  const std::string& message)
      // TODO: Column calculation assumes no unicode characters
      : str_(str), message_(message), column_(ptr - str.c_str()) {}

  explicit StringException(const StringException& other)
      : str_(other.str_), message_(other.message_), column_(other.column_) {}

  const std::string& GetString() const { return str_; }

  const std::string& GetMessage() const { return message_; }

  int64_t GetColumnOffset() const { return column_; }

 private:
  std::string str_;
  std::string message_;
  int64_t column_;
};

class LineException : public StringException {
 public:
  LineException(int line, const StringException& inner)
      : StringException(inner), line_(line) {}

  LineException(const LineException& other)
      : StringException(other), line_(other.line_) {}

  int GetLineOffset() const { return line_; }

 private:
  int line_;
};

class FileException : public LineException {
 public:
  FileException(const std::experimental::filesystem::path& path,
                const LineException& inner)
      : LineException(inner), path_(path) {}

  const std::experimental::filesystem::path& GetPath() const { return path_; }

  std::string what() const {
    std::ostringstream stream;
    stream << GetPath() << ':' << GetLineOffset() + 1 << ':'
           << GetColumnOffset() + 1 << ": error: " << GetMessage() << '\n'
           << GetString() << '\n';
    for (int i = 0; i < GetColumnOffset(); ++i) {
      stream << ' ';
    }
    stream << '^';
    return stream.str();
  }

 private:
  std::experimental::filesystem::path path_;
};

class StringParser {
 public:
  explicit StringParser(const std::string& str)
      : str_(str), ptr_(str.c_str()) {}

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
    unsigned int result = 0;
    const char* num_start = ptr_;
    const char* num_end = ptr_;
    while (*num_end >= '0' && *num_end <= '9') {
      ++num_end;
    }

    const char* ptr = num_end - 1;
    unsigned int power = 1;
    while (ptr >= num_start) {
      result += static_cast<unsigned int>(*ptr - 48) * power;
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
    return factor * static_cast<int>(ParseUInt());
  }

  float ParseFloat() {
    char* end;
    float x = strtof(ptr_, &end);
    if (ptr_ == end) throw StringException(str_, ptr_, "invalid float");
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
  const std::string& str_;
  const char* ptr_;
};

}  // namespace wavefront

#endif  // WAVEFRONT_PARSER_H_
