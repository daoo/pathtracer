#ifndef TRACE_SAMPLEBUFFER_H_
#define TRACE_SAMPLEBUFFER_H_

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
      : width_(width),
        height_(height),
        samples_(0),
        buffer_(width * height, glm::vec3(0, 0, 0)) {
    assert(width > 0 && height > 0);
  }

  unsigned int width() const { return width_; }
  unsigned int height() const { return height_; }

  unsigned int samples() const { return samples_; }

  void inc() { ++samples_; }

  const glm::vec3& get(unsigned int x, unsigned int y) const {
    return buffer_[y * width_ + x];
  }

  void add(unsigned int x, unsigned int y, const glm::vec3& v) {
    buffer_[y * width_ + x] += v;
  }

  const glm::vec3* data() const { return buffer_.data(); }

  /**
   * Append another buffer to this one.
   * @param other the other buffer, must be of the same size as this buffer
   */
  void append(const SampleBuffer& other) {
    assert(width() == other.width() && height() == other.height());
    auto it = buffer_.begin();
    auto io = other.buffer_.cbegin();
    while (it < buffer_.cend()) {
      *it += *io;
      ++it;
      ++io;
    }

    samples_ += other.samples();
  }

 private:
  unsigned int width_, height_;
  unsigned int samples_;

  std::vector<glm::vec3> buffer_;
};

/**
 * Write the average for each pixel in the buffer to PNG file.
 * @param file the PNG file, is overwritten if it already exists
 * @param buffer the buffer to write
 */
void write_image(const std::string& file, const SampleBuffer& buffer);
}  // namespace trace

#endif  // TRACE_SAMPLEBUFFER_H_
