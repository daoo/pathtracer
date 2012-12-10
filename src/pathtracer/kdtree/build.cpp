#include "build.hpp"

#include "math/tribox.hpp"
#include "pathtracer/kdtree/util.hpp"

using namespace math;
using namespace std;

namespace kdtree {
  namespace {
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

  void buildTreeSAH(KdTreeLinked::BuildIter iter, const Aabb& box,
      const vector<const Triangle*>& triangles) {
    float cost, split;
    vector<const Triangle*> left_triangles, right_triangles;
    Aabb left_box, right_box;

    findSplit(box, iter.axis(), triangles, cost, split,
        left_box, right_box, left_triangles, right_triangles);

    if (iter.depth() > 20 || cost > COST_INTERSECT * triangles.size()) {
      iter.leaf(triangles);
    } else {
      iter.split(split);
      buildTreeSAH(iter.left(), left_box, left_triangles);
      buildTreeSAH(iter.right(), right_box, right_triangles);
    }
  }

  void buildTreeNaive(KdTreeLinked::BuildIter iter, const Aabb& box,
      const vector<const Triangle*>& triangles) {
    if (iter.depth() >= 20 || triangles.size() <= 10) {
      iter.leaf(triangles);
    } else {
      float d = helpers::swizzle(box.center, iter.axis());

      Aabb left_box;
      Aabb right_box;

      helpers::aabbFromSplit(box, iter.axis(), d,
          left_box, right_box);

      vector<const Triangle*> left_triangles, right_triangles;

      intersectTest(left_box, right_box, triangles,
          left_triangles, right_triangles);

      iter.split(d);

      buildTreeNaive(iter.left(), left_box, left_triangles);
      buildTreeNaive(iter.right(), right_box, right_triangles);
    }
  }
}
