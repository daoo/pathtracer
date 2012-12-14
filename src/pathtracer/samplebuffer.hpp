#ifndef SAMPLEBUFFER_HPP_BIC38RBM
#define SAMPLEBUFFER_HPP_BIC38RBM

#include <boost/filesystem.hpp>
#include <glm/glm.hpp>
#include <string>
#include <vector>

namespace util
{
  /**
   * Pixel buffer stored in row major order.
   */
  class SampleBuffer
  {
    public:
      SampleBuffer(size_t w, size_t h)
        : m_width(w), m_height(h), m_samples(0), m_buffer(w * h)
      {
        assert(w > 0 && h > 0);
      }

      size_t width() const { return m_width; }
      size_t height() const { return m_height; }

      size_t size() const
      {
        return m_buffer.size();
      }

      size_t samples() const
      {
        return m_samples;
      }

      void increaseSamples()
      {
        ++m_samples;
      }

      const glm::vec3& at(size_t x, size_t y) const
      {
        return m_buffer[y * m_width + x];
      }

      void add(size_t x, size_t y, const glm::vec3& v)
      {
        m_buffer[y * m_width + x] += v;
      }

      const glm::vec3* data() const
      {
        return m_buffer.data();
      }

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
      size_t m_width, m_height;
      size_t m_samples;

      std::vector<glm::vec3> m_buffer;
  };

  void writeImage(const boost::filesystem::path&, const util::SampleBuffer&);
}

#endif /* end of include guard: SAMPLEBUFFER_HPP_BIC38RBM */
