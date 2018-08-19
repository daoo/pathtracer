#include <experimental/filesystem>

#include <cstdlib>
#include <iostream>
#include <string>

#include "trace/camera.h"
#include "trace/raytracer.h"
#include "trace/samplebuffer.h"
#include "trace/scene.h"
#include "util/clock.h"
#include "util/nicetime.h"
#include "wavefront/mtl.h"
#include "wavefront/obj.h"

using std::experimental::filesystem::path;

constexpr int OK = 0;
constexpr int ERROR_PARAMS = 1;
constexpr int ERROR_FILE_NOT_FOUND = 2;
constexpr int ERROR_PROGRAM = 3;

void program(const path& obj_file,
             const path& mtl_file,
             const path& out_file,
             unsigned int width,
             unsigned int height,
             unsigned int camera) {
  // Setup scene
  trace::Scene scene(wavefront::LoadObj(obj_file),
                     wavefront::LoadMtl(mtl_file));
  trace::Pinhole pinhole(
      scene.GetCameras()[camera],
      static_cast<float>(width) / static_cast<float>(height));

  trace::SampleBuffer buffer(width, height);

  trace::Raytracer raytracer;
  util::Clock clock;
  raytracer.Render(scene, pinhole, &buffer);
  double trace_time = clock.measure<double, std::ratio<1>>();
  std::cout << "Rendered in " << util::TimeAutoUnit(trace_time) << ".\n";

  write_image(out_file, buffer);
}

int main(int argc, char* argv[]) {
  if (argc != 6) {
    std::cerr << "Usage: " << argv[0] << " OBJ MTL OUT WIDTH HEIGHT\n";
    return ERROR_PARAMS;
  }

  const char* obj_file_str = argv[1];
  const char* mtl_file_str = argv[2];
  const char* out_file_str = argv[3];
  const char* width_str = argv[4];
  const char* height_str = argv[5];

  path obj_file(obj_file_str);
  path mtl_file(mtl_file_str);
  path out_file(out_file_str);
  int width = atoi(width_str);
  int height = atoi(height_str);

  if (width <= 0) {
    std::cerr << "Error: invalid width: " << width_str << '\n';
    return ERROR_PARAMS;
  }
  if (height <= 0) {
    std::cerr << "Error: invalid height: " << height_str << '\n';
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
    unsigned int camera = 0;
    program(obj_file, mtl_file, out_file, static_cast<unsigned int>(width),
            static_cast<unsigned int>(height), camera);
  } catch (const std::string& str) {
    std::cerr << str << '\n';
    return ERROR_PROGRAM;
  }

  return OK;
}
