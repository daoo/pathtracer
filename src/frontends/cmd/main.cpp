#include "thread.hpp"
#include "util/strings.hpp"

#include <iostream>

using namespace std;
using namespace util;

int main(int argc, char* argv[])
{
  if (argc != 7) {
    cerr << "Usage: pathtracer model.obj output-dir width height samples threads\n";
    return 1;
  }

  string obj_file = argv[1];
  string img_dir  = argv[2];

  unsigned int width   = parse<unsigned int>(argv[3]);
  unsigned int height  = parse<unsigned int>(argv[4]);
  unsigned int samples = parse<unsigned int>(argv[5]);
  unsigned int threads = parse<unsigned int>(argv[6]);

  try {
    program(obj_file, img_dir, width, height, 0, samples, threads);
  } catch (const string& str) {
    cerr << str;
  }

  return 0;
}
