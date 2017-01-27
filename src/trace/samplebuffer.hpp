#ifndef SAMPLEBUFFER_HPP_BIC38RBM
#define SAMPLEBUFFER_HPP_BIC38RBM

#include <glm/glm.hpp>
#include <string>
#include <vector>

namespace trace {
/**
 * Uncompressed, 2 dimensional RGB color buffer.
 * Stored in row major order thus column-first traversal has better locality.
 */
class SampleBuffer {
 public:
  /**
   * Construct new sample buffer with specific width and height.
   * All pixels are initialized to black (#000000).
   * @param width the width of the buffer, must be greater than 0
   * @param height the height of the buffer, must be greater than 0
   */
  SampleBuffer(unsigned int width, unsigned int height)
      : m_width(width),
        m_height(height),
        m_samples(0),
        m_buffer(width * height, glm::vec3(0, 0, 0)) {
    assert(width > 0 && height > 0);
  }

  unsigned int width() const { return m_width; }
  unsigned int height() const { return m_height; }

  unsigned int samples() const { return m_samples; }

  void inc() { ++m_samples; }

  const glm::vec3& get(unsigned int x, unsigned int y) const {
    return m_buffer[y * m_width + x];
  }

  void add(unsigned int x, unsigned int y, const glm::vec3& v) {
    m_buffer[y * m_width + x] += v;
  }

  const glm::vec3* data() const { return m_buffer.data(); }

  /**
   * Append another buffer to this one.
   * @param other the other buffer, must be of the same size as this buffer
   */
  void append(const SampleBuffer& other) {
    assert(width() == other.width() && height() == other.height());
    auto it = m_buffer.begin();
    auto io = other.m_buffer.cbegin();
    while (it < m_buffer.cend()) {
      *it += *io;
      ++it;
      ++io;
    }

    m_samples += other.samples();
  }

 private:
  unsigned int m_width, m_height;
  unsigned int m_samples;

  std::vector<glm::vec3> m_buffer;
};

/**
 * Write the average for each pixel in the buffer to PNG file.
 * @param file the PNG file, is overwritten if it already exists
 * @param buffer the buffer to write
 */
void write_image(const std::string& file, const SampleBuffer& buffer);
}

#endif /* end of include guard: SAMPLEBUFFER_HPP_BIC38RBM */
