#ifndef TRACE_TEXTURE_H_
#define TRACE_TEXTURE_H_

#include <glm/glm.hpp>
#include <string>
#include <vector>

namespace trace {
/**
 * RGB texture container.
 */
class Texture {
 public:
  Texture(unsigned int width,
          unsigned int height,
          const std::vector<glm::vec3>& image)
      : image_(image), width_(width), height_(height) {}

  const glm::vec3& sample(float x, float y) {
    assert(x >= 0.0f && x < 1.0f);
    assert(y >= 0.0f && y < 1.0f);
    return sample(static_cast<unsigned int>(x * width_),
                  static_cast<unsigned int>(y * height_));
  }

  const glm::vec3& sample(unsigned int x, unsigned int y) {
    assert(x < width_);
    assert(y < height_);
    return image_[x + y * width_];
  }

 private:
  /**
   * Vector containing the pixel colors stored in row-major order.
   */
  std::vector<glm::vec3> image_;
  unsigned int width_, height_;
};

/**
 * Load a texture from a file.
 */
Texture texture_load(const std::string&);
}  // namespace trace

#endif  // TRACE_TEXTURE_H_
