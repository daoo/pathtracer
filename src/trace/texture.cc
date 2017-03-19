#include "trace/texture.h"

#include <FreeImage.h>

namespace trace {
Texture texture_load(const std::string& file) {
  FIBITMAP* bitmap = FreeImage_Load(FIF_PNG, file.c_str());

  if (!bitmap) {
    std::string err = "Failed to load texture '";
    err += file;
    err += '\'';
    throw err;
  }

  FIBITMAP* rgbabitmap = FreeImage_ConvertTo32Bits(bitmap);
  FreeImage_Unload(bitmap);

  unsigned int width = FreeImage_GetWidth(rgbabitmap);
  unsigned int height = FreeImage_GetHeight(rgbabitmap);

  std::vector<glm::vec3> image;
  image.reserve(width * height);

  unsigned int BYTESPP =
      FreeImage_GetLine(rgbabitmap) / FreeImage_GetWidth(rgbabitmap);
  unsigned int i = 0;
  for (unsigned int y = 0; y < height; ++y) {
    BYTE* bits = FreeImage_GetScanLine(rgbabitmap, static_cast<int>(y));

    for (unsigned int x = 0; x < width; ++x) {
      glm::vec3& c = image[i];
      c.r = static_cast<float>(bits[FI_RGBA_RED]) / 255.0f;
      c.g = static_cast<float>(bits[FI_RGBA_GREEN]) / 255.0f;
      c.b = static_cast<float>(bits[FI_RGBA_BLUE]) / 255.0f;
      bits += BYTESPP;
      ++i;
    }
  }

  FreeImage_Unload(rgbabitmap);

  return Texture(width, height, image);
}
}  // namespace trace
