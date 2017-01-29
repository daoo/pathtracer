#ifndef PATHTRACER_HPP_NVGMZUSY
#define PATHTRACER_HPP_NVGMZUSY

#include "geometry/ray.h"
#include "kdtree/array.h"
#include "trace/camera.h"
#include "trace/fastrand.h"
#include "trace/light.h"
#include "trace/samplebuffer.h"
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
