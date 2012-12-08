#ifndef PATHTRACER_HPP_NVGMZUSY
#define PATHTRACER_HPP_NVGMZUSY

#include "math/ray.hpp"
#include "mcsampling.hpp"
#include "scene.hpp"

#include <glm/glm.hpp>

constexpr size_t PT_MAX_BOUNCES = 16;
constexpr float PT_EPSILON      = 0.00001f;

class Pathtracer {
  public:
    Pathtracer(size_t, size_t, const Scene&, size_t);
    ~Pathtracer();

    void tracePrimaryRays();

    glm::vec3 Li(const math::Ray&, const Intersection&);
    glm::vec3 Lenvironment(const math::Ray&);

    size_t samples() const { return m_samples; }
    size_t width() const { return m_iwidth; }
    size_t height() const { return m_iheight; }

    const std::vector<glm::vec3>& buffer() const {
      return m_buffer;
    }

  private:
    std::vector<glm::vec3> m_buffer;

    size_t m_samples;
    size_t m_iwidth, m_iheight;
    float m_fwidth, m_fheight;

    const Scene& m_scene;

    glm::vec3 m_camera_pos;
    glm::vec3 m_min_d;
    glm::vec3 m_dx, m_dy;
};

#endif /* end of include guard: PATHTRACER_HPP_NVGMZUSY */
