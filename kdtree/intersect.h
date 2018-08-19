#ifndef KDTREE_INTERSECT_H_
#define KDTREE_INTERSECT_H_

#include <vector>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/triangle.h"

namespace kdtree {

struct IntersectResults {
  std::vector<const geometry::Triangle*> left;
  std::vector<const geometry::Triangle*> plane;
  std::vector<const geometry::Triangle*> right;
};

inline IntersectResults PartitionTriangles(
    const geometry::Aabb& boundary,
    const std::vector<const geometry::Triangle*>& triangles,
    const geometry::Aap& plane) {
  IntersectResults results;
  results.left.reserve(triangles.size());
  results.right.reserve(triangles.size());
  for (const geometry::Triangle* triangle : triangles) {
    float clamped_min =
        boundary.GetClamped(triangle->GetMin())[plane.GetAxis()];
    float clamped_max =
        boundary.GetClamped(triangle->GetMax())[plane.GetAxis()];
    float plane_distance = plane.GetDistance();
    bool in_left = clamped_min < plane_distance;
    bool in_right = clamped_max > plane_distance;
    bool in_plane = !in_left && !in_right;
    if (in_left) results.left.emplace_back(triangle);
    if (in_right) results.right.emplace_back(triangle);
    if (in_plane) results.plane.emplace_back(triangle);
  }
  return results;
}

}  // namespace kdtree

#endif  // KDTREE_INTERSECT_H_
