#ifndef KDTREE_SAH_COMMON_H_
#define KDTREE_SAH_COMMON_H_

#include "geometry/split.h"
#include "kdtree/build_common.h"

namespace kdtree {

constexpr float COST_EMPTY = 0.01f;
constexpr float COST_TRAVERSE = 0.1f;
constexpr float COST_INTERSECT = 1.0f;

inline float LeafCostBound(size_t parent_count) {
  return COST_INTERSECT * parent_count;
}

inline float CalculateCost(float parent_area,
                           float left_area,
                           float right_area,
                           size_t left_count,
                           size_t right_count) {
  if (parent_area == 0 || left_area == 0 || right_area == 0) {
    return FLT_MAX;
  }

  float traverse =
      left_count == 0 || right_count == 0 ? COST_EMPTY : COST_TRAVERSE;
  float area_heuristic = (left_area * left_count + right_area * right_count);
  float intersect = COST_INTERSECT * area_heuristic / parent_area;
  return traverse + intersect;
}

enum Side { LEFT, RIGHT };

struct KdCost {
  float cost;
  kdtree::Side side;

  bool operator<(const KdCost& other) const { return cost < other.cost; }
};

inline KdCost CalculateCost(const geometry::Aabb& parent,
                            const geometry::Aap& plane,
                            size_t left_count,
                            size_t right_count,
                            size_t plane_count) {
  float parent_area = parent.GetSurfaceArea();
  geometry::AabbSplit split = geometry::Split(parent, plane);
  float left_area = split.left.GetSurfaceArea();
  float right_area = split.right.GetSurfaceArea();
  float plane_left =
      kdtree::CalculateCost(parent_area, left_area, right_area,
                            left_count + plane_count, right_count);
  float plane_right =
      kdtree::CalculateCost(parent_area, left_area, right_area, left_count,
                            right_count + plane_count);
  return plane_left <= plane_right ? KdCost{plane_left, kdtree::LEFT}
                                   : KdCost{plane_right, kdtree::RIGHT};
}

struct KdSplit {
  geometry::Aap plane;
  kdtree::KdCost cost;

  bool operator<(const KdSplit& other) const { return cost < other.cost; }
};

enum Type { START, PLANAR, END };

struct Event {
  geometry::Aap plane;
  Type type;

  bool operator<(const Event& other) const {
    return plane < other.plane || (plane == other.plane && type < other.type);
  }
};

inline void ListPerfectSplits(const geometry::Aabb& boundary,
                       const geometry::Triangle& triangle,
                       geometry::Axis axis,
                       std::set<Event>* splits) {
  float boundary_min = boundary.GetMin()[axis];
  float boundary_max = boundary.GetMax()[axis];
  float triangle_min = triangle.GetMin()[axis];
  float triangle_max = triangle.GetMax()[axis];
  float clamped_min = glm::clamp(triangle_min, boundary_min, boundary_max);
  float clamped_max = glm::clamp(triangle_max, boundary_min, boundary_max);
  if (clamped_min == clamped_max) {
    splits->insert({{axis, clamped_min}, PLANAR});
  } else {
    splits->insert({{axis, clamped_min}, START});
    splits->insert({{axis, clamped_max}, END});
  }
}

inline void ListPerfectSplits(const geometry::Aabb& boundary,
                       const geometry::Triangle& triangle,
                       std::set<Event>* splits) {
  ListPerfectSplits(boundary, triangle, geometry::X, splits);
  ListPerfectSplits(boundary, triangle, geometry::Y, splits);
  ListPerfectSplits(boundary, triangle, geometry::Z, splits);
}

inline std::set<Event> ListPerfectSplits(const KdBox& parent) {
  std::set<Event> splits;
  for (const geometry::Triangle* triangle : parent.triangles) {
    ListPerfectSplits(parent.boundary, *triangle, &splits);
  }
  return splits;
}

}  // namespace kdtree

#endif  // KDTREE_SAH_COMMON_H_
