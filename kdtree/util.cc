#include "kdtree/util.h"

#include <assert.h>
#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include "geometry/aabb.h"
#include "geometry/triangle.h"
#include "geometry/tribox.h"

using geometry::Aabb;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using std::vector;

namespace kdtree {
namespace {
struct AabbResults {
  Aabb left, right;
};

AabbResults split_aabb(const Aabb& parent, const Aap& plane) {
  float min = parent.GetMin()[plane.GetAxis()];
  float max = parent.GetMax()[plane.GetAxis()];
  float split = plane.GetDistance();
  float left_half_axis = (split - min) / 2.0f + glm::epsilon<float>();
  float right_half_axis = (max - split) / 2.0f + glm::epsilon<float>();
  assert(left_half_axis > 0 && right_half_axis > 0);

  glm::vec3 left_center(parent.GetCenter()), left_half(parent.GetHalf());
  left_center[plane.GetAxis()] = split - left_half_axis;
  left_half[plane.GetAxis()] = left_half_axis;

  glm::vec3 right_center(parent.GetCenter()), right_half(parent.GetHalf());
  right_center[plane.GetAxis()] = split + right_half_axis;
  right_half[plane.GetAxis()] = right_half_axis;

  return {Aabb(left_center, left_half), Aabb(right_center, right_half)};
}

struct IntersectResults {
  vector<const Triangle*> left;
  vector<const Triangle*> right;
};

IntersectResults intersect_test(const vector<const Triangle*>& triangles,
                                const Aabb& left_aabb,
                                const Aabb& right_aabb) {
  IntersectResults results;
  results.left.reserve(triangles.size());
  results.right.reserve(triangles.size());
  for (const Triangle* triangle : triangles) {
    bool in_left =
        tri_box_overlap(left_aabb, triangle->v0, triangle->v1, triangle->v2);
    bool in_right =
        tri_box_overlap(right_aabb, triangle->v0, triangle->v1, triangle->v2);
    assert(in_left || in_right);
    if (in_left) results.left.emplace_back(triangle);
    if (in_right) results.right.emplace_back(triangle);
  }

  results.left.shrink_to_fit();
  results.right.shrink_to_fit();
  return results;
}
}  // namespace

Split split_box(const Box& parent, const Aap& plane) {
  AabbResults aabbs = split_aabb(parent.boundary, plane);
  IntersectResults triangles =
      intersect_test(parent.triangles, aabbs.left, aabbs.right);
  return Split{plane, Box{aabbs.left, triangles.left},
               Box{aabbs.right, triangles.right}};
}
}  // namespace kdtree
