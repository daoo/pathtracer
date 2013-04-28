#ifndef SAMPLEBUFFER_HPP_BIC38RBM
#define SAMPLEBUFFER_HPP_BIC38RBM

#include <glm/glm.hpp>
#include <string>
#include <vector>

namespace trace
{
  /**
   * Uncompressed, 2 dimensional RGB color buffer.
   * Stored in row major order thus column-first traversal has better locality.
   */
  class SampleBuffer
  {
    public:
      /**
       * Construct new sample buffer with specific width and height.
       * All pixels are initialized to black (#000000).
       * @param width the width of the buffer, must be greater than 0
       * @param height the height of the buffer, must be greater than 0
       */
      SampleBuffer(unsigned int width, unsigned int height)
        : m_width(width)
        , m_height(height)
        , m_samples(0)
        , m_buffer(width * height, glm::vec3(0, 0, 0))
      {
        assert(width > 0 && height > 0);
      }

      /**
       * Get the width of the buffer.
       * @return the width of the buffer as an unsigned int, greater than zero
       */
      unsigned int width() const
      {
        return m_width;
      }

      /**
       * Get the height of the buffer.
       * @return the height of the buffer as an unsigned int, greater than zero
       */
      unsigned int height() const
      {
        return m_height;
      }

      /**
       * Return the number of samples in this buffer.
       * Can be zero if no samples have been added.
       * @return an unsigned int greater than or equal to zero
       */
      unsigned int samples() const
      {
        return m_samples;
      }

      /**
       * Increase the number of samples by one.
       */
      void inc()
      {
        ++m_samples;
      }

      /**
       * Retrive the RGB color stored at a certain location in the buffer.
       * @param x the x location, must be less than the width
       * @param y the y location, must be less than the height
       * @return the color at the position or black (#000000) if the location
       * havn't been added to yet.
       */
      const glm::vec3& get(unsigned int x, unsigned int y) const
      {
        return m_buffer[y * m_width + x];
      }

      /**
       * Add color sample to certain location in the buffer.
       * @param x the x location, must be less than the width
       * @param y the y location, must be less than the height
       */
      void add(unsigned int x, unsigned int y, const glm::vec3& v)
      {
        m_buffer[y * m_width + x] += v;
      }

      /**
       * Return a pointer to the underlying storage array.
       * For example this could be used to feed the buffer data to a OpenGL
       * texture.
       * @return a constant pointer to the underlying array, guaranteed to be
       * non-null
       */
      const glm::vec3* data() const
      {
        return m_buffer.data();
      }

      /**
       * Append another buffer to this one.
       * @param other the other buffer, must be of the same size as this buffer
       */
      void append(const SampleBuffer& other)
      {
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
  void writeImage(const std::string& file, const SampleBuffer& buffer);
}

#endif /* end of include guard: SAMPLEBUFFER_HPP_BIC38RBM */
