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

    unsigned int counter = 2;
    while (exists(ss.str())) {
      ss.str(string());
      ss << start << "_" << counter << ext;
      ++counter;
    }

    return ss.str();
  }
}
