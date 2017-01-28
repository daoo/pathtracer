#include "kdtree/surface_area_heuristic.hpp"

#include "geometry/aabb.hpp"
#include "geometry/bounding.hpp"
#include "geometry/triangle.hpp"
#include "kdtree/util.hpp"
#include <cassert>
#include <glm/glm.hpp>
#include <limits>
#include <vector>

namespace kdtree {
namespace {
constexpr float EPSILON = 0.00001f;

constexpr float COST_TRAVERSE = 0.3f;
constexpr float COST_INTERSECT = 1.0f;

float calculate_cost(
    const geometry::Aabb& box,
    const geometry::Aabb& left_box,
    const geometry::Aabb& right_box,
    const std::vector<const geometry::Triangle*>& left_triangles,
    const std::vector<const geometry::Triangle*>& right_triangles) {
  float area = surface_area(box);

  float left_area = surface_area(left_box);
  float left_count = left_triangles.size();

  float right_area = surface_area(right_box);
  float right_count = right_triangles.size();

  assert(left_area > 0 && right_area > 0);
  assert(right_count >= 0 && left_count >= 0);

  return COST_TRAVERSE +
         COST_INTERSECT * (left_area * left_count + right_area * right_count) /
             area;
}

void best(const geometry::Aabb& box,
          Axis axis,
          float split,
          const std::vector<const geometry::Triangle*>& triangles,
          float& best_cost,
          float& best_split,
          geometry::Aabb& best_left_box,
          geometry::Aabb& best_right_box,
          std::vector<const geometry::Triangle*>& best_left_triangles,
          std::vector<const geometry::Triangle*>& best_right_triangles) {
  geometry::Aabb left_box, right_box;
  split_aabb(box, axis, split, left_box, right_box);

  std::vector<const geometry::Triangle *> left_triangles, right_triangles;
  intersect_test(left_box, right_box, triangles, left_triangles,
                 right_triangles);

  float cost =
      calculate_cost(box, left_box, right_box, left_triangles, right_triangles);
  if (cost < best_cost) {
    best_cost = cost;
    best_split = split;

    best_left_box = left_box;
    best_right_box = right_box;

    best_left_triangles = left_triangles;
    best_right_triangles = right_triangles;
  }
}

void find_split(const geometry::Aabb& box,
                Axis axis,
                const std::vector<const geometry::Triangle*>& triangles,
                float& cost,
                float& split,
                geometry::Aabb& left_box,
                geometry::Aabb& right_box,
                std::vector<const geometry::Triangle*>& left_triangles,
                std::vector<const geometry::Triangle*>& right_triangles) {
  cost = std::numeric_limits<float>::max();
  split = 0;
  for (const geometry::Triangle* triangle : triangles) {
    assert(triangle != nullptr);

    glm::vec3 vmin, vmax;
    triangle_extremes(*triangle, vmin, vmax);
    float min = vmin[axis] - EPSILON;
    float max = vmax[axis] + EPSILON;

    best(box, axis, min, triangles, cost, split, left_box, right_box,
         left_triangles, right_triangles);
    best(box, axis, max, triangles, cost, split, left_box, right_box,
         left_triangles, right_triangles);
  }
}

void go(LinkedNode* node,
        unsigned int depth,
        Axis axis,
        const geometry::Aabb& box,
        const std::vector<const geometry::Triangle*>& triangles) {
  assert(node != nullptr);

  float cost, split;
  std::vector<const geometry::Triangle *> left_triangles, right_triangles;
  geometry::Aabb left_box, right_box;

  find_split(box, axis, triangles, cost, split, left_box, right_box,
             left_triangles, right_triangles);

  if (depth >= 20 || cost > COST_INTERSECT * triangles.size()) {
    node->type = LinkedNode::Leaf;
    node->leaf.triangles =
        new std::vector<const geometry::Triangle*>(triangles);
  } else {
    node->type = LinkedNode::NodeType::Split;
    node->split.axis = axis;
    node->split.distance = split;
    node->split.left = new LinkedNode;
    node->split.right = new LinkedNode;

    go(node->split.left, depth + 1, next_axis(axis), left_box, left_triangles);
    go(node->split.right, depth + 1, next_axis(axis), right_box,
       right_triangles);
  }
}
}

KdTreeLinked build_tree_sah(const std::vector<geometry::Triangle>& triangles) {
  std::vector<const geometry::Triangle*> ptrs;
  for (const geometry::Triangle& tri : triangles) {
    ptrs.push_back(&tri);
  }

  LinkedNode* root = new LinkedNode;
  go(root, 0, X, find_bounding(triangles), ptrs);

  return KdTreeLinked(root);
}
}
