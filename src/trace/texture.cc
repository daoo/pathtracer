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

  Texture texture;
  texture.width = FreeImage_GetWidth(rgbabitmap);
  texture.height = FreeImage_GetHeight(rgbabitmap);

  texture.image.reserve(texture.width * texture.height);

  unsigned int BYTESPP =
      FreeImage_GetLine(rgbabitmap) / FreeImage_GetWidth(rgbabitmap);
  for (unsigned int y = 0; y < texture.height; ++y) {
    BYTE* bits = FreeImage_GetScanLine(rgbabitmap, static_cast<int>(y));

    for (unsigned int x = 0; x < texture.width; ++x) {
      glm::vec3& c = texture_sample(texture, x, y);
      c.r = static_cast<float>(bits[FI_RGBA_RED]) / 255.0f;
      c.g = static_cast<float>(bits[FI_RGBA_GREEN]) / 255.0f;
      c.b = static_cast<float>(bits[FI_RGBA_BLUE]) / 255.0f;
      bits += BYTESPP;
    }
  }

  FreeImage_Unload(rgbabitmap);

  return texture;
}
}  // namespace trace