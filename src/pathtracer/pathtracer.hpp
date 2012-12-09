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
    Pathtracer(const Scene&, size_t, size_t, size_t);
    ~Pathtracer();

    void tracePrimaryRays(FastRand&, util::SampleBuffer&);

    glm::vec3 Li(FastRand&, const math::Ray&, const Intersection&);
    glm::vec3 Lenvironment(const math::Ray&);

  private:
    const Scene& m_scene;
    float m_fwidth, m_fheight;

    glm::vec3 m_camera_pos;
    glm::vec3 m_min_d;
    glm::vec3 m_dx, m_dy;
};

#endif /* end of include guard: PATHTRACER_HPP_NVGMZUSY */
