#ifndef INTERSECTION_HPP_B7YTSMBV
#define INTERSECTION_HPP_B7YTSMBV

#include "trace/material.hpp"
#include <glm/glm.hpp>

namespace trace {
struct Intersection {
  glm::vec3 position;
  glm::vec3 normal;
  const Material* material;
};
}

#endif /* end of include guard: INTERSECTION_HPP_B7YTSMBV */
