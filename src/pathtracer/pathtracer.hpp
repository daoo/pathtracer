#ifndef PATHTRACER_HPP_NVGMZUSY
#define PATHTRACER_HPP_NVGMZUSY

#include "fastrand.hpp"
#include "math/ray.hpp"
#include "mcsampling.hpp"
#include "samplebuffer.hpp"
#include "scene.hpp"

#include <glm/glm.hpp>

class Pathtracer
{
  public:
    Pathtracer(const Scene&, size_t, size_t, size_t);
    ~Pathtracer();

    void tracePrimaryRays(util::FastRand&, util::SampleBuffer&) const;

  private:
    const Scene& m_scene;
    float m_fwidth, m_fheight;

    glm::vec3 m_camera_pos;
    glm::vec3 m_min_d;
    glm::vec3 m_dx, m_dy;

    glm::vec3 incomingLight(util::FastRand&, const math::Ray&, const Intersection&) const;
    glm::vec3 environmentLight(const math::Ray&) const;
};

#endif /* end of include guard: PATHTRACER_HPP_NVGMZUSY */
