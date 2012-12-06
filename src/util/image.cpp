#include "image.hpp"

#include <FreeImage.h>
#include <sstream>

using namespace glm;
using namespace std;

namespace {
  constexpr float GAMMA_POWER = 1.0f / 2.2f;
}

void writeImage(const string& file, size_t w, size_t h, size_t samples, const vector<vec3>& data) {
  FIBITMAP* dib = FreeImage_Allocate(w, h, 32, FI_RGBA_RED_MASK, FI_RGBA_GREEN_MASK, FI_RGBA_BLUE_MASK);

  int bytespp = FreeImage_GetLine(dib) / FreeImage_GetWidth(dib);
  for (size_t y = 0; y < FreeImage_GetHeight(dib); ++y) {
    BYTE* bits = FreeImage_GetScanLine(dib, y);

    for (size_t x = 0; x < FreeImage_GetWidth(dib); ++x) {
      float r = glm::min(1.0f, pow(data[y * w + x].x / samples, GAMMA_POWER));
      float g = glm::min(1.0f, pow(data[y * w + x].y / samples, GAMMA_POWER));
      float b = glm::min(1.0f, pow(data[y * w + x].z / samples, GAMMA_POWER));
      bits[FI_RGBA_RED]   = BYTE(r * 255.0);
      bits[FI_RGBA_GREEN] = BYTE(g * 255.0);
      bits[FI_RGBA_BLUE]  = BYTE(b * 255.0);
      bits[FI_RGBA_ALPHA] = 255;

      bits += bytespp;
    }
  }

  if (!FreeImage_Save(FIF_PNG, dib, file.c_str(), 0)) {
    stringstream ss;
    ss << "Failed to save screenshot to file '" << file << "'";
    throw ss.str();
  }

  FreeImage_Unload(dib);
}
