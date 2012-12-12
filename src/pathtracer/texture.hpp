#ifndef TEXTURE_HPP_NJD06RG1
#define TEXTURE_HPP_NJD06RG1

#include <glm/glm.hpp>
#include <string>
#include <vector>

struct Texture {
  std::vector<glm::vec3> m_image;
  size_t m_width, m_height;
};

Texture textureLoad(Texture&, const std::string& filename);

#endif /* end of include guard: TEXTURE_HPP_NJD06RG1 */
