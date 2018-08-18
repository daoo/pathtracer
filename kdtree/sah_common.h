#ifndef KDTREE_SAH_COMMON_H_
#define KDTREE_SAH_COMMON_H_

namespace kdtree {

constexpr float COST_EMPTY = 0.01f;
constexpr float COST_TRAVERSE = 0.1f;
constexpr float COST_INTERSECT = 1.0f;

float LeafCostBound(size_t parent_count) {
  return COST_INTERSECT * parent_count;
}

float CalculateCost(float parent_area,
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

}  // namespace kdtree

#endif  // KDTREE_SAH_COMMON_H_
