#include "texture.hpp"

#include "FreeImage.h"

#include <iostream>

using namespace glm;
using namespace std;

Texture textureLoad(Texture& texture, const string& filename) {
  FIBITMAP* bitmap = FreeImage_Load(FIF_PNG, filename.c_str());

  if (!bitmap) {
    cout << "Failed to load texture " << filename << endl;
  }

  FIBITMAP *rgbabitmap = FreeImage_ConvertTo32Bits(bitmap);
  FreeImage_Unload(bitmap);

  texture.m_width  = FreeImage_GetWidth(rgbabitmap);
  texture.m_height = FreeImage_GetHeight(rgbabitmap);

  //int scan_width = FreeImage_GetPitch(rgbabitmap);
  //BYTE* data     = new BYTE[m_height*scan_width];

  texture.m_image.reserve(texture.m_width * texture.m_height);

  size_t bytespp = FreeImage_GetLine(rgbabitmap) / FreeImage_GetWidth(rgbabitmap);
  for (size_t y = 0; y < texture.m_height; ++y){
    BYTE* bits = FreeImage_GetScanLine(rgbabitmap, y);
    for (size_t x = 0; x < texture.m_width; ++x){
      size_t i = y * texture.m_width + x;
      texture.m_image[i].x = static_cast<float>(bits[FI_RGBA_RED])   / 255.0f;
      texture.m_image[i].y = static_cast<float>(bits[FI_RGBA_GREEN]) / 255.0f;
      texture.m_image[i].z = static_cast<float>(bits[FI_RGBA_BLUE])  / 255.0f;
      bits += bytespp;
    }
  }

  FreeImage_Unload(rgbabitmap);

  return texture;
}

