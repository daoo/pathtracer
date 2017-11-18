#ifndef GEOMETRY_TRIPLANE_H_
#define GEOMETRY_TRIPLANE_H_

#include <glm/glm.hpp>

namespace geometry {
class Aap;
bool tri_in_plane(const Aap& plane,
                  const glm::vec3& triv0,
                  const glm::vec3& triv1,
                  const glm::vec3& triv2);
}  // namespace geometry

#endif  // GEOMETRY_TRIPLANE_H_
