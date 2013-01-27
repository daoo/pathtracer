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

  Texture textureLoad(const std::string& filename);
}

#endif /* end of include guard: TEXTURE_HPP_NJD06RG1 */
