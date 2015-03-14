#ifndef TEXTURE_HPP_NJD06RG1
#define TEXTURE_HPP_NJD06RG1

#include <glm/glm.hpp>
#include <string>
#include <vector>

namespace trace
{
  /**
   * RGB texture container.
   */
  struct Texture
  {
    /**
     * Vector containing the pixel colors stored in row-major order.
     */
    std::vector<glm::vec3> image;
    unsigned int width, height;
  };

  inline glm::vec3 texture_sample(
      const Texture& texture,
      float x, float y)
  {
    assert(x >= 0.0f && x < 1.0f);
    assert(y >= 0.0f && y < 1.0f);
    return texture_sample(texture,
        static_cast<unsigned int>(x * texture.width),
        static_cast<unsigned int>(y * texture.height));
  }

  inline glm::vec3 texture_sample(
      const Texture& texture,
      unsigned int x,
      unsigned int y)
  {
    assert(x < texture.width);
    assert(y < texture.height);
    return texture.image[x + y * texture.width];
  }

  inline glm::vec3& texture_sample(
      Texture& texture,
      unsigned int x,
      unsigned int y)
  {
    assert(x < texture.width);
    assert(y < texture.height);
    return texture.image[x + y * texture.width];
  }

  /**
   * Load a texture from a file.
   */
  Texture texture_load(const std::string&);
}

#endif /* end of include guard: TEXTURE_HPP_NJD06RG1 */
