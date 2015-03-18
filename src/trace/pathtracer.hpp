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
  struct Pinhole
  {
    float width, height;

    glm::vec3 position;
    glm::vec3 mind;
    glm::vec3 dx, dy;
  };

  Pinhole new_pinhole(
      const Camera& camera,
      unsigned int width,
      unsigned int height);

  inline Ray pinhole_ray(const Pinhole& pinhole, float x, float y)
  {
    return {
      pinhole.position,
      normalize(pinhole.mind + x * pinhole.dx + y * pinhole.dy)
    };
  }

  void pathtrace(
      const kdtree::KdTree& kdtree,
      const std::vector<SphereLight>& lights,
      const Pinhole& pinhole,
      FastRand& rand,
      SampleBuffer& buffer);
}

#endif /* end of include guard: PATHTRACER_HPP_NVGMZUSY */
