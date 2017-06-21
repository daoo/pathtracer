#ifndef GEOMETRY_SPLIT_H_
#define GEOMETRY_SPLIT_H_

#include <glm/glm.hpp>
#include <cassert>

#include "geometry/aabb.h"
#include "geometry/aap.h"

namespace geometry {

class Aap;

struct AabbSplit {
  Aabb left, right;
};

AabbSplit split(const Aabb& aabb, const Aap& plane) {
  float left_half_axis =
      (plane.GetDistance() - aabb.GetMin()[plane.GetAxis()]) / 2.0f;
  float right_half_axis =
      (aabb.GetMax()[plane.GetAxis()] - plane.GetDistance()) / 2.0f;
  assert(left_half_axis >= 0 && right_half_axis >= 0);

  glm::vec3 left_center(aabb.GetCenter()), left_half(aabb.GetHalf());
  left_center[plane.GetAxis()] = plane.GetDistance() - left_half_axis;
  left_half[plane.GetAxis()] = left_half_axis;

  glm::vec3 right_center(aabb.GetCenter()), right_half(aabb.GetHalf());
  right_center[plane.GetAxis()] = plane.GetDistance() + right_half_axis;
  right_half[plane.GetAxis()] = right_half_axis;

  return {Aabb(left_center, left_half), Aabb(right_center, right_half)};
}

}  // namespace geometry

#endif  // GEOMETRY_SPLIT_H_
