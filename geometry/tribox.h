#ifndef GEOMETRY_TRIBOX_H_
#define GEOMETRY_TRIBOX_H_

#include <glm/glm.hpp>

namespace geometry {
class Aabb;

bool tri_box_overlap(const Aabb& aabb,
                     const glm::vec3& triv0,
                     const glm::vec3& triv1,
                     const glm::vec3& triv2);
}  // namespace geometry

#endif  // GEOMETRY_TRIBOX_H_
