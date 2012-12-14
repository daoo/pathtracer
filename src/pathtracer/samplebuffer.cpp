#include "samplebuffer.hpp"

#include <FreeImage.h>
#include <sstream>

using namespace boost::filesystem;
using namespace glm;
using namespace std;

namespace
{
  constexpr float GAMMA_POWER = 1.0f / 2.2f;

  float gammaCorrect(float x) {
    return std::min(1.0f, pow(x, GAMMA_POWER));
  }
}

namespace util
{
  void writeImage(const path& file, const SampleBuffer& buffer)
  {
    FIBITMAP* dib = FreeImage_Allocate(buffer.width(), buffer.height(),
        32, FI_RGBA_RED_MASK, FI_RGBA_GREEN_MASK, FI_RGBA_BLUE_MASK);

    int bytespp = FreeImage_GetLine(dib) / FreeImage_GetWidth(dib);
    for (size_t y = 0; y < FreeImage_GetHeight(dib); ++y) {
      BYTE* bits = FreeImage_GetScanLine(dib, y);

      for (size_t x = 0; x < FreeImage_GetWidth(dib); ++x) {
        float r = gammaCorrect(buffer.at(x, y).r / buffer.samples());
        float g = gammaCorrect(buffer.at(x, y).g / buffer.samples());
        float b = gammaCorrect(buffer.at(x, y).b / buffer.samples());
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
}
