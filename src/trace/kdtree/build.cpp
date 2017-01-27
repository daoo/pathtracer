#include "trace/kdtree/build.hpp"

#include "geometry/bounding.hpp"
#include "geometry/tribox.hpp"
#include "trace/kdtree/util.hpp"
#include <glm/glm.hpp>

using namespace glm;
using namespace std;

namespace trace
{
  namespace kdtree
  {
    namespace
    {
      constexpr float EPSILON = 0.00001f;

      void split_aabb(
          const Aabb& box,
          Axis axis,
          float split,
          Aabb& left,
          Aabb& right)
      {
        left  = box;
        right = box;

        float split_clamped = glm::clamp(
            split,
            box.center[axis] - box.half[axis],
            box.center[axis] + box.half[axis]);

        float min = box.center[axis] - box.half[axis];
        float max = box.center[axis] + box.half[axis];

        float lh = (split_clamped - min) / 2.0f + EPSILON;
        float rh = (max - split_clamped) / 2.0f + EPSILON;

        left.half[axis]   = lh;
        left.center[axis] = split_clamped - lh;

        right.half[axis]   = rh;
        right.center[axis] = split_clamped + rh;
      }

      constexpr float COST_TRAVERSE  = 0.3f;
      constexpr float COST_INTERSECT = 1.0f;

      float calculate_cost(
          const Aabb& box,
          const Aabb& left_box,
          const Aabb& right_box,
          const vector<const Triangle*>& left_triangles,
          const vector<const Triangle*>& right_triangles)
      {
        float area = surface_area(box);

        float left_area  = surface_area(left_box);
        float left_count = left_triangles.size();

        float right_area  = surface_area(right_box);
        float right_count = right_triangles.size();

        assert(left_area > 0 && right_area > 0);
        assert(right_count >= 0 && left_count >= 0);

        return COST_TRAVERSE + COST_INTERSECT *
          (left_area * left_count + right_area * right_count) / area;
      }

      void intersect_test(
          const Aabb& left_box,
          const Aabb& right_box,
          const vector<const Triangle*>& triangles,
          vector<const Triangle*>& left_triangles,
          vector<const Triangle*>& right_triangles)
      {

        left_triangles.reserve(triangles.size());
        right_triangles.reserve(triangles.size());
        for (const Triangle* tri : triangles) {
          if (tri_box_overlap(left_box, tri->v0, tri->v1, tri->v2)) {
            left_triangles.push_back(tri);
          }

          if (tri_box_overlap(right_box, tri->v0, tri->v1, tri->v2)) {
            right_triangles.push_back(tri);
          }
        }

        assert(left_triangles.size() + right_triangles.size() >= triangles.size());

        left_triangles.shrink_to_fit();
        right_triangles.shrink_to_fit();
      }

      void best(
          const Aabb& box,
          Axis axis,
          float split,
          const vector<const Triangle*>& triangles,
          float& best_cost,
          float& best_split,
          Aabb& best_left_box,
          Aabb& best_right_box,
          vector<const Triangle*>& best_left_triangles,
          vector<const Triangle*>& best_right_triangles)
      {
        Aabb left_box, right_box;
        split_aabb(box, axis, split, left_box, right_box);

        vector<const Triangle*> left_triangles, right_triangles;
        intersect_test(
            left_box,
            right_box,
            triangles,
            left_triangles,
            right_triangles);

        float cost = calculate_cost(
            box,
            left_box,
            right_box,
            left_triangles,
            right_triangles);
        if (cost < best_cost) {
          best_cost  = cost;
          best_split = split;

          best_left_box  = left_box;
          best_right_box = right_box;

          best_left_triangles  = left_triangles;
          best_right_triangles = right_triangles;
        }
      }

      void find_split(
          const Aabb& box,
          Axis axis,
          const vector<const Triangle*>& triangles,
          float& cost,
          float& split,
          Aabb& left_box,
          Aabb& right_box,
          vector<const Triangle*>& left_triangles,
          vector<const Triangle*>& right_triangles)
      {
        cost  = FLT_MAX;
        split = 0;
        for (const Triangle* triangle : triangles) {
          assert(triangle != nullptr);

          vec3 vmin, vmax;
          triangle_extremes(*triangle, vmin, vmax);
          float min = vmin[axis] - EPSILON;
          float max = vmax[axis] + EPSILON;

          best(
              box,
              axis,
              min,
              triangles,
              cost,
              split,
              left_box,
              right_box,
              left_triangles,
              right_triangles);
          best(
              box,
              axis,
              max,
              triangles,
              cost,
              split,
              left_box,
              right_box,
              left_triangles,
              right_triangles);
        }
      }
    }

    void go_sah(
        LinkedNode* node,
        unsigned int depth,
        Axis axis,
        const Aabb& box,
        const vector<const Triangle*>& triangles)
    {
      assert(node != nullptr);

      float cost, split;
      vector<const Triangle*> left_triangles, right_triangles;
      Aabb left_box, right_box;

      find_split(
          box,
          axis,
          triangles,
          cost,
          split,
          left_box,
          right_box,
          left_triangles,
          right_triangles);

      if (depth >= 20 || cost > COST_INTERSECT * triangles.size()) {
        node->type           = LinkedNode::Leaf;
        node->leaf.triangles = new vector<const Triangle*>(triangles);
      } else {
        node->type           = LinkedNode::NodeType::Split;
        node->split.axis     = axis;
        node->split.distance = split;
        node->split.left     = new LinkedNode;
        node->split.right    = new LinkedNode;

        go_sah(
            node->split.left,
            depth + 1,
            next_axis(axis),
            left_box,
            left_triangles);
        go_sah(
            node->split.right,
            depth + 1,
            next_axis(axis),
            right_box,
            right_triangles);
      }
    }

    void go_naive(
        LinkedNode* node,
        unsigned int depth,
        Axis axis,
        const Aabb& box,
        const vector<const Triangle*>& triangles)
    {
      assert(node != nullptr);

      if (depth >= 20 || triangles.size() <= 10) {
        node->type           = LinkedNode::NodeType::Leaf;
        node->leaf.triangles = new vector<const Triangle*>(triangles);
      } else {
        float split = box.center[axis];

        Aabb left_box;
        Aabb right_box;

        split_aabb(box, axis, split, left_box, right_box);

        vector<const Triangle*> left_triangles, right_triangles;

        intersect_test(
            left_box,
            right_box,
            triangles,
            left_triangles,
            right_triangles);

        node->type           = LinkedNode::NodeType::Split;
        node->split.axis     = axis;
        node->split.distance = split;
        node->split.left     = new LinkedNode;
        node->split.right    = new LinkedNode;

        go_naive(
            node->split.left,
            depth + 1,
            next_axis(axis),
            left_box,
            left_triangles);
        go_naive(
            node->split.right,
            depth + 1,
            next_axis(axis),
            right_box,
            right_triangles);
      }
    }

    KdTreeLinked build_tree_sah(const vector<Triangle>& triangles)
    {
      vector<const Triangle*> ptrs;
      for (const Triangle& tri : triangles) {
        ptrs.push_back(&tri);
      }

      LinkedNode* root = new LinkedNode;
      go_sah(root, 0, X, find_bounding(triangles), ptrs);

      return KdTreeLinked(root);
    }

    KdTreeLinked build_tree_naive(const vector<Triangle>& triangles)
    {
      vector<const Triangle*> ptrs;
      for (const Triangle& tri : triangles) {
        ptrs.push_back(&tri);
      }

      LinkedNode* root = new LinkedNode;
      go_naive(root, 0, X, find_bounding(triangles), ptrs);

      return KdTreeLinked(root);
    }
  }
}
