#include "util/path.h"

#include <sstream>

using std::string;
using std::stringstream;
using std::experimental::filesystem::exists;
using std::experimental::filesystem::path;

namespace util {
path next_free_name(const path& dir, const string& name, const string& ext) {
  const string start = (dir / name).string();

  stringstream ss;
  ss << start << ext;

  unsigned int counter = 2;
  while (exists(ss.str())) {
    ss.str(string());
    ss << start << "_" << counter << ext;
    ++counter;
  }

  return ss.str();
}

string nice_name(const path& file,
                 unsigned int width,
                 unsigned int height,
                 unsigned int samples) {
  stringstream name;
  name << file.stem() << "_" << width << "x" << height << "_" << samples;
  return name.str();
}
}  // namespace util
