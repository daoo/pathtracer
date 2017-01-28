#include "trace/samplebuffer.hpp"

#include <FreeImage.h>

using namespace glm;
using namespace std;

namespace trace {
namespace {
constexpr float GAMMA_POWER = 1.0f / 2.2f;

float gamma_correct(float x) {
  return glm::min(1.0f, pow(x, GAMMA_POWER));
}
}

void write_image(const string& file, const SampleBuffer& buffer) {
  const float samples = static_cast<float>(buffer.samples());
  FIBITMAP* bitmap = FreeImage_Allocate(static_cast<int>(buffer.width()),
                                        static_cast<int>(buffer.height()), 24);

  for (unsigned int y = 0; y < FreeImage_GetHeight(bitmap); ++y) {
    for (unsigned int x = 0; x < FreeImage_GetWidth(bitmap); ++x) {
      vec3 pixel = buffer.get(x, y);
      RGBQUAD rgb;
      rgb.rgbRed = static_cast<BYTE>(gamma_correct(pixel.r / samples) * 255.0f);
      rgb.rgbGreen =
          static_cast<BYTE>(gamma_correct(pixel.g / samples) * 255.0f);
      rgb.rgbBlue =
          static_cast<BYTE>(gamma_correct(pixel.b / samples) * 255.0f);
      FreeImage_SetPixelColor(bitmap, x, y, &rgb);
    }
  }

  if (!FreeImage_Save(FIF_PNG, bitmap, file.c_str(), 0)) {
    string err = "Failed to save screenshot to file '";
    err += file;
    err += '\'';
    throw err;
  }

  FreeImage_Unload(bitmap);
}
}
