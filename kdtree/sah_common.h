#ifndef KDTREE_SAH_COMMON_H_
#define KDTREE_SAH_COMMON_H_

#include "geometry/split.h"

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

}  // namespace kdtree

#endif  // KDTREE_SAH_COMMON_H_
