#include "path.hpp"

#include <sstream>

using namespace boost::filesystem;
using namespace std;

namespace util
{
  path nextFreeName(const path& dir, const string& name, const string& ext)
  {
    const string start = (dir / name).string();

    stringstream ss;
    ss << start << ext;

    size_t counter = 2;
    while (exists(ss.str())) {
      ss.clear();
      ss << start << "_" << counter << ext;
    }

    return ss.str();
  }
}
