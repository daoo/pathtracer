#ifndef SAMPLEBUFFER_HPP_BIC38RBM
#define SAMPLEBUFFER_HPP_BIC38RBM

#include <boost/filesystem.hpp>
#include <glm/glm.hpp>
#include <string>
#include <vector>

/**
  * Pixel buffer stored in row major order.
  */
class SampleBuffer
{
  public:
    SampleBuffer(unsigned int w, unsigned int h)
      : m_width(w), m_height(h), m_samples(0), m_buffer(w * h)
    {
      assert(w > 0 && h > 0);
    }

    unsigned int width() const { return m_width; }
    unsigned int height() const { return m_height; }

    unsigned int size() const
    {
      return m_buffer.size();
    }

    unsigned int samples() const
    {
      return m_samples;
    }

    void increaseSamples()
    {
      ++m_samples;
    }

    const glm::vec3& at(unsigned int x, unsigned int y) const
    {
      return m_buffer[y * m_width + x];
    }

    void add(unsigned int x, unsigned int y, const glm::vec3& v)
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
    unsigned int m_width, m_height;
    unsigned int m_samples;

    std::vector<glm::vec3> m_buffer;
};

void writeImage(const boost::filesystem::path&, const SampleBuffer&);

#endif /* end of include guard: SAMPLEBUFFER_HPP_BIC38RBM */
