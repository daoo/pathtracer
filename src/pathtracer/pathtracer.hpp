#ifndef PATHTRACER_HPP_NVGMZUSY
#define PATHTRACER_HPP_NVGMZUSY

#include "math/ray.hpp"
#include "mcsampling.hpp"
#include "scene.hpp"
#include "util/fastrand.hpp"
#include "util/samplebuffer.hpp"

#include <glm/glm.hpp>

class Pathtracer {
  public:
    Pathtracer(const Pathtracer&) = default;

    Pathtracer(size_t, size_t, const Scene&, size_t);
    ~Pathtracer();

    void tracePrimaryRays();

    glm::vec3 Li(const math::Ray&, const Intersection&);
    glm::vec3 Lenvironment(const math::Ray&);

    size_t samples() const { return m_buffer.samples(); }
    size_t width() const { return m_buffer.width(); }
    size_t height() const { return m_buffer.height(); }

    const util::SampleBuffer& buffer() const {
      return m_buffer;
    }

    friend Pathtracer merge(const Pathtracer&, const Pathtracer&);

  private:
    util::SampleBuffer m_buffer;

    float m_fwidth, m_fheight;

    const Scene& m_scene;

    glm::vec3 m_camera_pos;
    glm::vec3 m_min_d;
    glm::vec3 m_dx, m_dy;

    FastRand m_fastrand;
};

#endif /* end of include guard: PATHTRACER_HPP_NVGMZUSY */
