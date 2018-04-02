#ifndef KDTREE_SAH_COMMON_H_
#define KDTREE_SAH_COMMON_H_

namespace kdtree {

constexpr float COST_TRAVERSE = 0.01f;
constexpr float COST_INTERSECT = 1.0f;
constexpr float COST_EMPTY = 0.8f;

float LeafCostBound(size_t parent_count) {
  return COST_INTERSECT * parent_count;
}

float CalculateCost(float parent_area,
                    float left_area,
                    float right_area,
                    size_t left_count,
                    size_t right_count) {
  float factor = left_count == 0 || right_count == 0 ? COST_EMPTY : 1.0f;
  float intersect =
      (left_area * left_count + right_area * right_count) / parent_area;
  return factor * COST_TRAVERSE + COST_INTERSECT * intersect;
}

}  // namespace kdtree

#endif  // KDTREE_SAH_COMMON_H_
