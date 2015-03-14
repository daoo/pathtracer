#include "trace/samplebuffer.hpp"

#include <FreeImage.h>

using namespace glm;
using namespace std;

namespace trace
{
  namespace
  {
    constexpr float GAMMA_POWER = 1.0f / 2.2f;

    float gamma_correct(float x)
    {
      return glm::min(1.0f, pow(x, GAMMA_POWER));
    }

    vec3 gamma_correct(const vec3& v)
    {
      return vec3(gamma_correct(v.r), gamma_correct(v.g), gamma_correct(v.b));
    }
  }

  void write_image(const string& file, const SampleBuffer& buffer)
  {
    const float samples = static_cast<float>(buffer.samples());
    FIBITMAP* dib = FreeImage_Allocate(buffer.width(), buffer.height(),
        32, FI_RGBA_RED_MASK, FI_RGBA_GREEN_MASK, FI_RGBA_BLUE_MASK);

    const unsigned int BYTESPP =
      FreeImage_GetLine(dib) / FreeImage_GetWidth(dib);
    for (unsigned int y = 0; y < FreeImage_GetHeight(dib); ++y) {
      BYTE* bits = FreeImage_GetScanLine(dib, y);

      for (unsigned int x = 0; x < FreeImage_GetWidth(dib); ++x) {
        vec3 c = gamma_correct(buffer.get(x, y) / samples) * 255.0f;
        bits[FI_RGBA_RED]   = static_cast<BYTE>(c.r);
        bits[FI_RGBA_GREEN] = static_cast<BYTE>(c.g);
        bits[FI_RGBA_BLUE]  = static_cast<BYTE>(c.b);
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
