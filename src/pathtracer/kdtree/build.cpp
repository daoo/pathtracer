#include "build.hpp"

#include "math/tribox.hpp"
#include "pathtracer/kdtree/util.hpp"

using namespace math;
using namespace std;

namespace kdtree {
  namespace {
    Axis next(Axis axis) {
      return static_cast<Axis>((static_cast<size_t>(axis) + 1) % 3);
    }

    constexpr float COST_TRAVERSE  = 0.3f;
    constexpr float COST_INTERSECT = 1.0f;

    float calculateCost(const Aabb& box,
        const Aabb& leftBox,
        const Aabb& rightBox,
        const vector<const Triangle*>& leftTriangles,
        const vector<const Triangle*>& rightTriangles) {
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
        vector<const Triangle*>& rightTriangles) {

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
        vector<const Triangle*>& bestRightTriangles) {
      Aabb left_box, right_box;
      helpers::aabbFromSplit(box, axis, split, left_box, right_box);

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
        vector<const Triangle*>& rightTriangles) {
      cost  = FLT_MAX;
      split = 0;
      for (const Triangle* triangle : triangles) {
        float min, max;
        helpers::triangleExtremes(*triangle, axis, min, max);

        best(box, axis, min, triangles, cost, split, leftBox, rightBox,
            leftTriangles, rightTriangles);
        best(box, axis, max, triangles, cost, split, leftBox, rightBox,
            leftTriangles, rightTriangles);
      }
    }
  }

  void buildTreeSAH(KdTreeLinked::Node* node, size_t depth, Axis axis, const Aabb& box,
      const vector<const Triangle*>& triangles) {
    float cost, split;
    vector<const Triangle*> left_triangles, right_triangles;
    Aabb left_box, right_box;

    findSplit(box, axis, triangles, cost, split,
        left_box, right_box, left_triangles, right_triangles);

    if (depth > 20 || cost > COST_INTERSECT * triangles.size()) {
      node->m_type             = KdTreeLinked::Node::Leaf;
      node->m_leaf.m_triangles = new vector<const Triangle*>(triangles);
    } else {
      node->m_type             = KdTreeLinked::Node::Split;
      node->m_split.m_axis     = axis;
      node->m_split.m_distance = split;
      node->m_split.m_left     = new KdTreeLinked::Node;
      node->m_split.m_right    = new KdTreeLinked::Node;

      buildTreeSAH(node->m_split.m_left, depth + 1, next(axis),
          left_box, left_triangles);
      buildTreeSAH(node->m_split.m_left, depth + 1, next(axis),
          right_box, right_triangles);
    }
  }

  void buildTreeNaive(KdTreeLinked::Node* node, size_t depth, Axis axis, const Aabb& box,
      const vector<const Triangle*>& triangles) {
    if (depth >= 20 || triangles.size() <= 10) {
      node->m_type             = KdTreeLinked::Node::Leaf;
      node->m_leaf.m_triangles = new vector<const Triangle*>(triangles);
    } else {
      float split = helpers::swizzle(box.center, axis);

      Aabb left_box;
      Aabb right_box;

      helpers::aabbFromSplit(box, axis, split, left_box, right_box);

      vector<const Triangle*> left_triangles, right_triangles;

      intersectTest(left_box, right_box, triangles,
          left_triangles, right_triangles);

      node->m_type             = KdTreeLinked::Node::Split;
      node->m_split.m_axis     = axis;
      node->m_split.m_distance = split;
      node->m_split.m_left     = new KdTreeLinked::Node;
      node->m_split.m_right    = new KdTreeLinked::Node;

      buildTreeNaive(node->m_split.m_left, depth + 1, next(axis),
          left_box, left_triangles);
      buildTreeNaive(node->m_split.m_right, depth + 1, next(axis),
          right_box, right_triangles);
    }
  }
}
