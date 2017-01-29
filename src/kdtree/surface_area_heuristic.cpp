#include "kdtree/surface_area_heuristic.hpp"

#include "geometry/aabb.hpp"
#include "geometry/bounding.hpp"
#include "geometry/triangle.hpp"
#include "kdtree/util.hpp"
#include <cassert>
#include <cstddef>
#include <glm/glm.hpp>
#include <tuple>
#include <vector>

namespace kdtree {
namespace {
constexpr float COST_TRAVERSE = 0.3f;
constexpr float COST_INTERSECT = 1.0f;
constexpr float EPSILON = 0.00001f;

struct CostSplit {
  Split split;
  float cost;
};

std::tuple<float, float> triangle_axis_extremes(
    const geometry::Triangle& triangle,
    Axis axis) {
  return std::make_tuple(
      glm::min(triangle.v0[axis],
               glm::min(triangle.v1[axis], triangle.v2[axis])),
      glm::max(triangle.v0[axis],
               glm::max(triangle.v1[axis], triangle.v2[axis])));
}

float calculate_cost(const Box& parent, const Split& split) {
  float parent_area = parent.boundary.surface_area();
  float left_area = split.left.boundary.surface_area();
  float left_count = split.left.triangles.size();
  float right_area = split.right.boundary.surface_area();
  float right_count = split.right.triangles.size();
  float intersect =
      (left_area * left_count + right_area * right_count) / parent_area;
  return COST_TRAVERSE + COST_INTERSECT * intersect;
}

CostSplit split(const Box& parent, Axis axis, float distance) {
  Split split = split_box(parent, axis, distance);
  float cost = calculate_cost(parent, split);
  return CostSplit{split, cost};
}

const CostSplit& get_best(const CostSplit& a, const CostSplit& b) {
  return a.cost <= b.cost ? a : b;
}

CostSplit find_best(const Box& parent, Axis axis) {
  assert(parent.triangles.size() > 0);
  CostSplit best;
  {
    std::tuple<float, float> extremes =
        triangle_axis_extremes(*parent.triangles[0], axis);
    float min = std::get<0>(extremes) - EPSILON;
    float max = std::get<1>(extremes) + EPSILON;

    CostSplit split_min = split(parent, axis, min);
    CostSplit split_max = split(parent, axis, max);
    best = get_best(split_min, split_max);
  }
  for (size_t i = 1; i < parent.triangles.size(); ++i) {
    std::tuple<float, float> extremes =
        triangle_axis_extremes(*parent.triangles[i], axis);
    float min = std::get<0>(extremes) - EPSILON;
    float max = std::get<1>(extremes) + EPSILON;

    CostSplit split_min = split(parent, axis, min);
    CostSplit split_max = split(parent, axis, max);
    best = get_best(best, get_best(split_min, split_max));
  }
  return best;
}

void go(LinkedNode* node, unsigned int depth, Axis axis, const Box& parent) {
  assert(node != nullptr);
  if (depth >= 20 || parent.triangles.empty()) {
    node->type = LinkedNode::Leaf;
    node->leaf.triangles =
        new std::vector<const geometry::Triangle*>(parent.triangles);
    return;
  }

  CostSplit split = find_best(parent, axis);
  float leaf_cost = COST_INTERSECT * parent.triangles.size();
  if (split.cost > leaf_cost) {
    node->type = LinkedNode::Leaf;
    node->leaf.triangles =
        new std::vector<const geometry::Triangle*>(parent.triangles);
  } else {
    node->type = LinkedNode::NodeType::Split;
    node->split.axis = axis;
    node->split.distance = split.split.distance;
    node->split.left = new LinkedNode;
    node->split.right = new LinkedNode;

    go(node->split.left, depth + 1, next_axis(axis), split.split.left);
    go(node->split.right, depth + 1, next_axis(axis), split.split.right);
  }
}
}

KdTreeLinked build_tree_sah(const std::vector<geometry::Triangle>& triangles) {
  std::vector<const geometry::Triangle*> ptrs;
  for (const geometry::Triangle& tri : triangles) {
    ptrs.push_back(&tri);
  }

  LinkedNode* root = new LinkedNode;
  go(root, 0, X, Box{find_bounding(triangles), ptrs});

  return KdTreeLinked(root);
}
}
