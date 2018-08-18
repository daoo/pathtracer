#ifndef TRACE_PATHTRACER_H_
#define TRACE_PATHTRACER_H_

#include <vector>

namespace geometry {
struct Ray;
}  // namespace geometry

namespace kdtree {
class KdTree;
}  // namespace kdtree

namespace trace {
class FastRand;
class SampleBuffer;
class SphereLight;
struct Pinhole;

void pathtrace(const kdtree::KdTree& kdtree,
               const std::vector<SphereLight>& lights,
               const Pinhole& pinhole,
               unsigned int bounces,
               FastRand* rand,
               SampleBuffer* buffer);
}  // namespace trace

#endif  // TRACE_PATHTRACER_H_
