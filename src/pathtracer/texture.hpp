#ifndef TEXTURE_HPP_NJD06RG1
#define TEXTURE_HPP_NJD06RG1

#include <glm/glm.hpp>
#include <string>
#include <vector>

struct Texture
{
  std::vector<glm::vec3> image;
  size_t width, height;
};

Texture textureLoad(Texture&, const std::string& filename);

#endif /* end of include guard: TEXTURE_HPP_NJD06RG1 */
