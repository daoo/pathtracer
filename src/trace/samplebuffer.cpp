#include "trace/samplebuffer.hpp"

#include <FreeImage.h>

using namespace glm;
using namespace std;

namespace trace
{
  namespace
  {
    constexpr float GAMMA_POWER = 1.0f / 2.2f;

    constexpr float min(float a, float b)
    {
      return a < b ? a : b;
    }

    float gammaCorrect(float x)
    {
      return min(1.0f, pow(x, GAMMA_POWER));
    }
  }

  void writeImage(const string& file, const SampleBuffer& buffer)
  {
    FIBITMAP* dib = FreeImage_Allocate(buffer.width(), buffer.height(),
        32, FI_RGBA_RED_MASK, FI_RGBA_GREEN_MASK, FI_RGBA_BLUE_MASK);

    const unsigned int BYTESPP =
      FreeImage_GetLine(dib) / FreeImage_GetWidth(dib);
    for (unsigned int y = 0; y < FreeImage_GetHeight(dib); ++y) {
      BYTE* bits = FreeImage_GetScanLine(dib, y);

      for (unsigned int x = 0; x < FreeImage_GetWidth(dib); ++x) {
        const float r = gammaCorrect(buffer.at(x, y).r / buffer.samples());
        const float g = gammaCorrect(buffer.at(x, y).g / buffer.samples());
        const float b = gammaCorrect(buffer.at(x, y).b / buffer.samples());
        bits[FI_RGBA_RED]   = BYTE(r * 255.0);
        bits[FI_RGBA_GREEN] = BYTE(g * 255.0);
        bits[FI_RGBA_BLUE]  = BYTE(b * 255.0);
        bits[FI_RGBA_ALPHA] = 255;

        bits += BYTESPP;
      }
    }

    if (!FreeImage_Save(FIF_PNG, dib, file.c_str(), 0)) {
      string err = "Failed to save screenshot to file '";
      err += file;
      err += '\'';
      throw err;
    }

    FreeImage_Unload(dib);
  }
}
