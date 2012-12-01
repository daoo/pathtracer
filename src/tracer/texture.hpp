#ifndef TEXTURE_HPP_NJD06RG1
#define TEXTURE_HPP_NJD06RG1

#include <glm/glm.hpp>
#include <string>

class Texture {
  public:
    glm::vec3* m_image;
    int m_width, m_height;

    void load(const std::string& filename);
};

#endif /* end of include guard: TEXTURE_HPP_NJD06RG1 */
