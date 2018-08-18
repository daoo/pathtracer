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
    const std::vector<const geometry::Triangle*>& triangles,
    const geometry::Aap& plane) {
  IntersectResults results;
  results.left.reserve(triangles.size());
  results.right.reserve(triangles.size());
  for (const geometry::Triangle* triangle : triangles) {
    float triangle_min = triangle->GetMin()[plane.GetAxis()];
    float triangle_max = triangle->GetMax()[plane.GetAxis()];
    float plane_distance = plane.GetDistance();
    bool in_left = triangle_min < plane_distance;
    bool in_right = triangle_max > plane_distance;
    bool in_plane = !in_left && !in_right;
    if (in_left) results.left.emplace_back(triangle);
    if (in_right) results.right.emplace_back(triangle);
    if (in_plane) results.plane.emplace_back(triangle);
  }
  return results;
}

}  // namespace kdtree

#endif  // KDTREE_INTERSECT_H_
