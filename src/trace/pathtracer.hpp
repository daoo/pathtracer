#ifndef PATHTRACER_HPP_NVGMZUSY
#define PATHTRACER_HPP_NVGMZUSY

#include "trace/camera.hpp"
#include "trace/fastrand.hpp"
#include "trace/kdtree/tree.hpp"
#include "trace/light.hpp"
#include "trace/samplebuffer.hpp"
#include <glm/glm.hpp>
#include <vector>

namespace trace
{
  class Pathtracer
  {
    public:
      Pathtracer(
          const Camera& camera,
          const kdtree::KdTree& kdtree,
          const std::vector<SphereLight> lights,
          unsigned int width,
          unsigned int height);

      ~Pathtracer();

      void trace(FastRand&, SampleBuffer&) const;

    private:
      const kdtree::KdTree& m_kdtree;
      const std::vector<SphereLight> m_lights;
      float m_fwidth, m_fheight;

      glm::vec3 m_camera_pos;
      glm::vec3 m_min_d;
      glm::vec3 m_dx, m_dy;
  };
}

#endif /* end of include guard: PATHTRACER_HPP_NVGMZUSY */
