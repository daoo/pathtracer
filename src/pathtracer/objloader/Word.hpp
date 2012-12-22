#ifndef WORD_HPP_Y7LXGQBT
#define WORD_HPP_Y7LXGQBT

#include <array>
#include <string>

namespace objloader
{
  struct Word
  {
    const std::string& str;
    size_t begin, end;
  };

  bool empty(const Word& word)
  {
    return word.begin == word.end;
  }

  size_t size(const Word& word)
  {
    return word.end - word.begin;
  }

  const char* c_str(const Word& word)
  {
    return word.str.c_str() + word.begin;
  }

  std::string str(const Word& word)
  {
    assert(word.begin < word.str.size());
    assert(word.end <= word.str.size());
    return word.str.substr(word.begin, size(word));
  }

  bool equal(const Word& word, const std::string& other)
  {
    if (size(word) == other.size()) {
      size_t i = word.begin;
      size_t j = 0;
      while (i < size(word)) {
        if (tolower(word.str[i]) != tolower(other[j]))
          return false;
        ++i;
        ++j;
      }

      return true;
    }

    return false;
  }

  int toInt(const Word& word)
  {
    return empty(word) ? 0 : atoi(c_str(word));
  }

  float toFloat(const Word& word)
  {
    return strtof(c_str(word), nullptr);
  }

  void parseFacePoint(const Word& word, std::array<int, 3>& output)
  {
    assert(!empty(word));

    std::array<size_t, 3> starts {{ word.begin, word.end, word.end }};
    std::array<size_t, 3> ends {{ word.end, word.end, word.end }};
    for (size_t i = word.begin, j = 0; i < word.end; ++i) {
      if (word.str[i] == '/') {
        ends[j]       = i;
        starts[j + 1] = i + 1;
        ++j;
      }
    }

    output[0] = toInt(Word{word.str, starts[0], ends[0]});
    output[1] = toInt(Word{word.str, starts[1], ends[1]});
    output[2] = toInt(Word{word.str, starts[2], ends[2]});
  }

  Word getWord(const std::string& str, size_t begin)
  {
    assert(!str.empty());

    size_t i = begin;
    while (i < str.size()) {
      char c = str[i];
      if (c != ' ' && c != '\t')
        break;
      ++i;
    }

    size_t j = i;
    while (j < str.size()) {
      char c = str[j];
      if (c == ' ' || c == '\t' || c == '\n')
        break;
      ++j;
    }

    return {str, i, j};
  }
}

#endif /* end of include guard: WORD_HPP_Y7LXGQBT */
