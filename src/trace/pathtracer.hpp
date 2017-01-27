#ifndef PATHTRACER_HPP_NVGMZUSY
#define PATHTRACER_HPP_NVGMZUSY

#include "geometry/ray.hpp"
#include "kdtree/array.hpp"
#include "trace/camera.hpp"
#include "trace/fastrand.hpp"
#include "trace/light.hpp"
#include "trace/samplebuffer.hpp"
#include <glm/glm.hpp>
#include <vector>

namespace trace {
void pathtrace(const kdtree::KdTreeArray& kdtree,
               const std::vector<SphereLight>& lights,
               const Pinhole& pinhole,
               FastRand& rand,
               SampleBuffer& buffer);
}

#endif /* end of include guard: PATHTRACER_HPP_NVGMZUSY */
