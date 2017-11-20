#ifndef KDTREE_INTERSECT_H_
#define KDTREE_INTERSECT_H_

#include <vector>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/triangle.h"

namespace kdtree {

struct IntersectResults {
  std::vector<const geometry::Triangle*> left;
  std::vector<const geometry::Triangle*> right;
};

IntersectResults intersect_test(
    const std::vector<const geometry::Triangle*>& triangles,
    const geometry::Aabb& left_aabb,
    const geometry::Aabb& right_aabb) {
  IntersectResults results;
  results.left.reserve(triangles.size());
  results.right.reserve(triangles.size());
  for (const geometry::Triangle* triangle : triangles) {
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

IntersectResults intersect_test(
    const std::vector<const geometry::Triangle*>& triangles,
    const geometry::Aap& plane) {
  IntersectResults results;
  results.left.reserve(triangles.size());
  results.right.reserve(triangles.size());
  for (const geometry::Triangle* triangle : triangles) {
    float triangle_min = triangle->GetMin()[plane.GetAxis()];
    float triangle_max = triangle->GetMax()[plane.GetAxis()];
    float plane_distance = plane.GetDistance();
    bool in_left = triangle_min <= plane_distance;
    bool in_right = triangle_max >= plane_distance;
    assert(in_left || in_right);
    if (in_left) results.left.emplace_back(triangle);
    if (in_right) results.right.emplace_back(triangle);
  }
  return results;
}

}  // namespace kdtree

#endif  // KDTREE_INTERSECT_H_
