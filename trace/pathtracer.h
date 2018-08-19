#ifndef TRACE_PATHTRACER_H_
#define TRACE_PATHTRACER_H_

#include <glm/glm.hpp>

#include "trace/fastrand.h"

namespace kdtree {
class KdTree;
}  // namespace kdtree

namespace geometry {
struct Ray;
}  // namespace geometry

namespace trace {
class Material;
class SampleBuffer;
class Scene;
class SphereLight;
struct Pinhole;

class Pathtracer {
 public:
  explicit Pathtracer(size_t max_bounces) : max_bounces_(max_bounces) {}

  glm::vec3 Trace(const Scene& scene, const geometry::Ray& ray);

  void Render(const Scene& scene, const Pinhole& pinhole, SampleBuffer* buffer);

 private:
  glm::vec3 EnvironmentContribution(const geometry::Ray& ray) const;

  glm::vec3 LightContribution(const Scene& scene,
                              const Material* material,
                              const glm::vec3& target,
                              const glm::vec3& offset,
                              const glm::vec3& wi,
                              const glm::vec3& n,
                              const SphereLight& light);

  glm::vec3 Trace(const Scene& scene,
                  const geometry::Ray& ray,
                  glm::vec3 radiance,
                  glm::vec3 transport,
                  unsigned int bounce);

  size_t max_bounces_;
  FastRand rand_;
};

}  // namespace trace

#endif  // TRACE_PATHTRACER_H_
