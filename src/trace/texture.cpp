#include "texture.hpp"

#include <FreeImage.h>
#include <iostream>

using namespace glm;
using namespace std;

namespace trace
{
  Texture textureLoad(const string& filename)
  {
    FIBITMAP* bitmap = FreeImage_Load(FIF_PNG, filename.c_str());

    if (!bitmap) {
      cout << "Failed to load texture " << filename << endl;
    }

    FIBITMAP *rgbabitmap = FreeImage_ConvertTo32Bits(bitmap);
    FreeImage_Unload(bitmap);

    Texture texture;
    texture.width  = FreeImage_GetWidth(rgbabitmap);
    texture.height = FreeImage_GetHeight(rgbabitmap);

    texture.image.reserve(texture.width * texture.height);

    const unsigned int BYTESPP =
      FreeImage_GetLine(rgbabitmap) / FreeImage_GetWidth(rgbabitmap);
    for (unsigned int y = 0; y < texture.height; ++y){
      BYTE* bits = FreeImage_GetScanLine(rgbabitmap, y);

      for (unsigned int x = 0; x < texture.width; ++x){
        glm::vec3& c = texture.image[y * texture.width + x];
        c.r = static_cast<float>(bits[FI_RGBA_RED])   / 255.0f;
        c.g = static_cast<float>(bits[FI_RGBA_GREEN]) / 255.0f;
        c.b = static_cast<float>(bits[FI_RGBA_BLUE])  / 255.0f;
        bits += BYTESPP;
      }
    }

    FreeImage_Unload(rgbabitmap);

    return texture;
  }
}
