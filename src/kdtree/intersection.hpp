#ifndef INTERSECTION_HPP_B7YTSMBV
#define INTERSECTION_HPP_B7YTSMBV

#include <glm/glm.hpp>

namespace trace {
namespace kdtree {
struct Intersection {
  glm::vec3 position;
  glm::vec3 normal;
  const void* tag;
};
}
}

#endif /* end of include guard: INTERSECTION_HPP_B7YTSMBV */
