#include <experimental/filesystem>

#include <cstdlib>
#include <iostream>
#include <sstream>
#include <string>

#include "pathtracer/thread.h"

using std::experimental::filesystem::path;

constexpr int OK = 0;
constexpr int ERROR_PARAMS = 1;
constexpr int ERROR_FILE_NOT_FOUND = 2;
constexpr int ERROR_PROGRAM = 3;

int main(int argc, char* argv[]) {
  if (argc != 8) {
    std::cerr << "Usage: " << argv[0]
              << " OBJ MTL OUT WIDTH HEIGHT SAMPLES JOBS\n";
    return ERROR_PARAMS;
  }

  const char* obj_file_str = argv[1];
  const char* mtl_file_str = argv[2];
  const char* out_file_str = argv[3];
  const char* width_str = argv[4];
  const char* height_str = argv[5];
  const char* samples_str = argv[6];
  const char* jobs_str = argv[7];

  path obj_file(obj_file_str);
  path mtl_file(mtl_file_str);
  path out_file(out_file_str);
  int width = atoi(width_str);
  int height = atoi(height_str);
  int samples = atoi(samples_str);
  int jobs = atoi(jobs_str);

  if (width <= 0) {
    std::cerr << "Error: invalid width: " << width_str << '\n';
    return ERROR_PARAMS;
  }
  if (height <= 0) {
    std::cerr << "Error: invalid height: " << height_str << '\n';
    return ERROR_PARAMS;
  }
  if (samples <= 0) {
    std::cerr << "Error: invalid sample count: " << samples_str << '\n';
    return ERROR_PARAMS;
  }
  if (jobs <= 0) {
    std::cerr << "Error: invalid job count: " << jobs_str << '\n';
    return ERROR_PARAMS;
  }
  if (!exists(obj_file)) {
    std::cerr << "Error: file " << obj_file << " does not exist.\n";
    return ERROR_FILE_NOT_FOUND;
  }
  if (!exists(mtl_file)) {
    std::cerr << "Error: file " << mtl_file << " does not exist.\n";
    return ERROR_FILE_NOT_FOUND;
  }
  if (out_file.empty()) {
    std::cerr << "Error: empty output file path.\n";
    return ERROR_FILE_NOT_FOUND;
  }

  try {
    program(obj_file, mtl_file, out_file, static_cast<unsigned int>(width),
            static_cast<unsigned int>(height), 0,
            static_cast<unsigned int>(samples),
            static_cast<unsigned int>(jobs));
  } catch (const std::string& str) {
    std::cerr << str << '\n';
    return ERROR_PROGRAM;
  }

  return OK;
}
