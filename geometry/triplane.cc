#include "geometry/triplane.h"

#include "geometry/aap.h"

namespace geometry {
bool tri_in_plane(const Aap& plane,
                  const glm::vec3& triv0,
                  const glm::vec3& triv1,
                  const glm::vec3& triv2) {
  return triv0[plane.GetAxis()] == plane.GetDistance() &&
         triv1[plane.GetAxis()] == plane.GetDistance() &&
         triv2[plane.GetAxis()] == plane.GetDistance();
}
}  // namespace geometry
