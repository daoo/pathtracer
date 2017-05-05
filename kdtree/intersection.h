#ifndef KDTREE_INTERSECTION_H_
#define KDTREE_INTERSECTION_H_

#include <glm/glm.hpp>

namespace kdtree {
struct Intersection {
  glm::vec3 position;
  glm::vec3 normal;
  const void* tag;
};
}  // namespace kdtree

#endif  // KDTREE_INTERSECTION_H_
