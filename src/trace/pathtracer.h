#ifndef TRACE_PATHTRACER_H_
#define TRACE_PATHTRACER_H_

#include <glm/glm.hpp>
#include <vector>

#include "geometry/ray.h"
#include "kdtree/linked.h"
#include "trace/camera.h"
#include "trace/fastrand.h"
#include "trace/light.h"
#include "trace/samplebuffer.h"

namespace trace {
void pathtrace(const kdtree::KdTreeLinked& kdtree,
               const std::vector<SphereLight>& lights,
               const Pinhole& pinhole,
               FastRand& rand,
               SampleBuffer& buffer);
}

#endif  // TRACE_PATHTRACER_H_
