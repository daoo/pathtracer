#ifndef PATHTRACER_HPP_NVGMZUSY
#define PATHTRACER_HPP_NVGMZUSY

#include "math/ray.hpp"
#include "trace/fastrand.hpp"
#include "trace/mcsampling.hpp"
#include "trace/samplebuffer.hpp"
#include "trace/scene.hpp"
#include <glm/glm.hpp>

namespace trace
{
  class Pathtracer
  {
    public:
      Pathtracer(const Scene&, unsigned int, unsigned int, unsigned int);
      ~Pathtracer();

      void trace(FastRand&, SampleBuffer&) const;

    private:
      const Scene& m_scene;
      float m_fwidth, m_fheight;

      glm::vec3 m_camera_pos;
      glm::vec3 m_min_d;
      glm::vec3 m_dx, m_dy;
  };
}

#endif /* end of include guard: PATHTRACER_HPP_NVGMZUSY */
