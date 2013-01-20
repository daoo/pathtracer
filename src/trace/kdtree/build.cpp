#include "build.hpp"

#include "math/tribox.hpp"
#include "trace/kdtree/util.hpp"
#include <glm/glm.hpp>

using namespace glm;
using namespace math;
using namespace std;

namespace trace
{
  namespace kdtree
  {
    namespace
    {
      constexpr float EPSILON = 0.00001f;

      void triangleExtremes(const Triangle& tri, Axis axis,
          float& min, float& max)
      {
        float a = tri.v0[axis];
        float b = tri.v1[axis];
        float c = tri.v2[axis];

        min = glm::min(glm::min(a, b), c);
        max = glm::max(glm::max(a, b), c);
      }

      void aabbFromSplit(const math::Aabb& box,
          Axis axis, float split, math::Aabb& left, math::Aabb& right)
      {
        left  = box;
        right = box;

        float splitClamped = glm::clamp(
            split,
            box.center[axis] - box.half[axis],
            box.center[axis] + box.half[axis]);

        float min = box.center[axis] - box.half[axis];
        float max = box.center[axis] + box.half[axis];

        float lh = (splitClamped - min) / 2.0f + EPSILON;
        float rh = (max - splitClamped) / 2.0f + EPSILON;

        left.half[axis]   = lh;
        left.center[axis] = splitClamped - lh;

        right.half[axis]   = rh;
        right.center[axis] = splitClamped + rh;
      }

      constexpr float COST_TRAVERSE  = 0.3f;
      constexpr float COST_INTERSECT = 1.0f;

      float calculateCost(const Aabb& box,
          const Aabb& leftBox,
          const Aabb& rightBox,
          const vector<const Triangle*>& leftTriangles,
          const vector<const Triangle*>& rightTriangles)
      {
        float area = surfaceArea(box);

        float left_area  = surfaceArea(leftBox);
        float left_count = leftTriangles.size();

        float right_area  = surfaceArea(rightBox);
        float right_count = rightTriangles.size();

        assert(left_area > 0 && right_area > 0);
        assert(right_count >= 0 && left_count >= 0);

        return COST_TRAVERSE + COST_INTERSECT *
          (left_area * left_count + right_area * right_count) / area;
      }

      void intersectTest(const Aabb& leftBox,
          const Aabb& rightBox,
          const vector<const Triangle*>& triangles,
          vector<const Triangle*>& leftTriangles,
          vector<const Triangle*>& rightTriangles)
      {

        leftTriangles.reserve(triangles.size());
        rightTriangles.reserve(triangles.size());
        for (const Triangle* tri : triangles) {
          if (triBoxOverlap(leftBox, tri->v0, tri->v1, tri->v2)) {
            leftTriangles.push_back(tri);
          }

          if (triBoxOverlap(rightBox, tri->v0, tri->v1, tri->v2)) {
            rightTriangles.push_back(tri);
          }
        }

        assert(leftTriangles.size() + rightTriangles.size() >= triangles.size());

        leftTriangles.shrink_to_fit();
        rightTriangles.shrink_to_fit();
      }

      void best(const Aabb& box, Axis axis, float split,
          const vector<const Triangle*>& triangles,
          float& bestCost, float& bestSplit,
          Aabb& bestLeftBox, Aabb& bestRightBox,
          vector<const Triangle*>& bestLeftTriangles,
          vector<const Triangle*>& bestRightTriangles)
      {
        Aabb left_box, right_box;
        aabbFromSplit(box, axis, split, left_box, right_box);

        vector<const Triangle*> left_triangles, right_triangles;
        intersectTest(left_box, right_box, triangles,
            left_triangles, right_triangles);

        float cost = calculateCost(box, left_box, right_box,
            left_triangles, right_triangles);
        if (cost < bestCost) {
          bestCost  = cost;
          bestSplit = split;

          bestLeftBox  = left_box;
          bestRightBox = right_box;

          bestLeftTriangles  = left_triangles;
          bestRightTriangles = right_triangles;
        }
      }

      void findSplit(const Aabb& box, Axis axis,
          const vector<const Triangle*>& triangles,
          float& cost, float& split,
          Aabb& leftBox, Aabb& rightBox,
          vector<const Triangle*>& leftTriangles,
          vector<const Triangle*>& rightTriangles)
      {
        cost  = FLT_MAX;
        split = 0;
        for (const Triangle* triangle : triangles) {
          assert(triangle != nullptr);

          float min, max;
          triangleExtremes(*triangle, axis, min, max);
          min -= EPSILON;
          max += EPSILON;

          best(box, axis, min, triangles, cost, split, leftBox, rightBox,
              leftTriangles, rightTriangles);
          best(box, axis, max, triangles, cost, split, leftBox, rightBox,
              leftTriangles, rightTriangles);
        }
      }
    }

    void buildTreeSAH(KdTreeLinked::Node* node, unsigned int depth,
        Axis axis, const Aabb& box, const vector<const Triangle*>& triangles)
    {
      assert(node != nullptr);

      float cost, split;
      vector<const Triangle*> left_triangles, right_triangles;
      Aabb left_box, right_box;

      findSplit(box, axis, triangles, cost, split,
          left_box, right_box, left_triangles, right_triangles);

      if (depth >= 20 || cost > COST_INTERSECT * triangles.size()) {
        node->type           = KdTreeLinked::Node::Leaf;
        node->leaf.triangles = new vector<const Triangle*>(triangles);
      } else {
        node->type           = KdTreeLinked::Node::Split;
        node->split.axis     = axis;
        node->split.distance = split;
        node->split.left     = new KdTreeLinked::Node;
        node->split.right    = new KdTreeLinked::Node;

        buildTreeSAH(node->split.left, depth + 1, nextAxis(axis),
            left_box, left_triangles);
        buildTreeSAH(node->split.right, depth + 1, nextAxis(axis),
            right_box, right_triangles);
      }
    }

    void buildTreeNaive(KdTreeLinked::Node* node, unsigned int depth, Axis axis,
        const Aabb& box, const vector<const Triangle*>& triangles)
    {
      assert(node != nullptr);

      if (depth >= 20 || triangles.size() <= 10) {
        node->type             = KdTreeLinked::Node::Leaf;
        node->leaf.triangles = new vector<const Triangle*>(triangles);
      } else {
        float split = box.center[axis];

        Aabb left_box;
        Aabb right_box;

        aabbFromSplit(box, axis, split, left_box, right_box);

        vector<const Triangle*> left_triangles, right_triangles;

        intersectTest(left_box, right_box, triangles,
            left_triangles, right_triangles);

        node->type           = KdTreeLinked::Node::Split;
        node->split.axis     = axis;
        node->split.distance = split;
        node->split.left     = new KdTreeLinked::Node;
        node->split.right    = new KdTreeLinked::Node;

        buildTreeNaive(node->split.left, depth + 1, nextAxis(axis),
            left_box, left_triangles);
        buildTreeNaive(node->split.right, depth + 1, nextAxis(axis),
            right_box, right_triangles);
      }
    }
  }
}
