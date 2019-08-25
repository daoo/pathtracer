#ifndef KDTREE_SAH_COST_H_
#define KDTREE_SAH_COST_H_

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/split.h"

namespace kdtree {
constexpr float COST_EMPTY_FACTOR = 0.8f;
constexpr float COST_TRAVERSE = 0.1f;
constexpr float COST_INTERSECT = 1.0f;

float CalculateSahCost(float probability_left,
                       float probability_right,
                       size_t number_left,
                       size_t number_right) {
  assert(probability_left >= 0);
  assert(probability_right >= 0);
  assert(probability_left > 0 || probability_right > 0);
  float empty_factor =
      number_left == 0 || number_right == 0 ? COST_EMPTY_FACTOR : 1.0f;
  float intersect_cost = COST_INTERSECT * (probability_left * number_left +
                                           probability_right * number_right);
  return empty_factor * (COST_TRAVERSE + intersect_cost);
}

enum Side { LEFT, RIGHT };

struct Cost {
  float cost;
  Side side;

  bool operator==(const Cost& other) const {
    return cost == other.cost && side == other.side;
  }
  bool operator<(const Cost& other) const {
    return cost < other.cost && side < other.side;
  }
};

Cost CalculateSahCost(const geometry::Aabb& parent,
                      const geometry::Aap& plane,
                      size_t left_count,
                      size_t right_count,
                      size_t plane_count) {
  assert(parent.GetSurfaceArea() > 0.0f);
  geometry::AabbSplit split = geometry::Split(parent, plane);
  if (split.left.GetVolume() <= 0.0f) return Cost{FLT_MAX, LEFT};
  if (split.right.GetVolume() <= 0.0f) return Cost{FLT_MAX, RIGHT};

  float surface_area_parent = parent.GetSurfaceArea();
  float surface_area_left = split.left.GetSurfaceArea();
  float surface_area_right = split.right.GetSurfaceArea();

  float probability_left = surface_area_left / surface_area_parent;
  float probability_right = surface_area_right / surface_area_parent;

  float cost_plane_left =
      CalculateSahCost(probability_left, probability_right,
                       left_count + plane_count, right_count);
  float cost_plane_right =
      CalculateSahCost(probability_left, probability_right, left_count,
                       right_count + plane_count);

  return cost_plane_left <= cost_plane_right ? Cost{cost_plane_left, LEFT}
                                             : Cost{cost_plane_right, RIGHT};
}
}  // namespace kdtree

#endif  // KDTREE_SAH_COST_H_
