#ifndef SAH_HPP_ID3KV0HO
#define SAH_HPP_ID3KV0HO

#include "math/aabb.hpp"
#include "math/tribox.hpp"
#include "pathtracer/kdtree/util.hpp"
#include "pathtracer/triangle.hpp"

#include <array>
#include <cassert>
#include <vector>

namespace kdtree {
  namespace detail {
    constexpr float COST_TRAVERSE  = 0.3f;
    constexpr float COST_INTERSECT = 1.0f;

    inline float calculateCost(const math::Aabb& box,
        const math::Aabb& leftBox,
        const math::Aabb& rightBox,
        const std::vector<Triangle>& leftTriangles,
        const std::vector<Triangle>& rightTriangles) {
      float area = math::surfaceArea(box);

      float left_area  = math::surfaceArea(leftBox);
      float left_count = leftTriangles.size();

      float right_area  = math::surfaceArea(rightBox);
      float right_count = rightTriangles.size();

      assert(left_area > 0 && right_area > 0);
      assert(right_count >= 0 && left_count >= 0);

      return COST_TRAVERSE + COST_INTERSECT *
        (left_area * left_count + right_area * right_count) / area;
    }

    inline void intersectTest(const math::Aabb& leftBox,
        const math::Aabb& rightBox,
        const std::vector<Triangle>& triangles,
        std::vector<Triangle>& leftTriangles,
        std::vector<Triangle>& rightTriangles) {

      leftTriangles.reserve(triangles.size());
      rightTriangles.reserve(triangles.size());
      for (const Triangle& tri : triangles) {
        if (triBoxOverlap(leftBox, tri.v0, tri.v1, tri.v2)) {
          leftTriangles.push_back(tri);
        }

        if (triBoxOverlap(rightBox, tri.v0, tri.v1, tri.v2)) {
          rightTriangles.push_back(tri);
        }
      }

      assert(leftTriangles.size() + rightTriangles.size() >= triangles.size());

      leftTriangles.shrink_to_fit();
      rightTriangles.shrink_to_fit();
    }

    inline void best(const math::Aabb& box, Axis axis, float split,
        const std::vector<Triangle>& triangles,
        float& bestCost, float& bestSplit,
        math::Aabb& bestLeftBox, math::Aabb& bestRightBox,
        std::vector<Triangle>& bestLeftTriangles,
        std::vector<Triangle>& bestRightTriangles) {
      math::Aabb left_box, right_box;
      helpers::aabbFromSplit(box, axis, split, left_box, right_box);

      std::vector<Triangle> left_triangles, right_triangles;
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

        assert(bestLeftTriangles.size() + bestRightTriangles.size() >= triangles.size());
      }
    }

    inline void findSplit(const math::Aabb& box, Axis axis,
        const std::vector<Triangle>& triangles,
        float& cost, float& split,
        math::Aabb& leftBox, math::Aabb& rightBox,
        std::vector<Triangle>& leftTriangles,
        std::vector<Triangle>& rightTriangles) {
      cost  = FLT_MAX;
      split = 0;
      for (const Triangle& triangle : triangles) {
        float min, max;
        helpers::triangleExtremes(triangle, axis, min, max);

        best(box, axis, min, triangles, cost, split, leftBox, rightBox,
            leftTriangles, rightTriangles);
        best(box, axis, max, triangles, cost, split, leftBox, rightBox,
            leftTriangles, rightTriangles);
      }
    }
  }

  template <typename Iter>
  void buildTreeSAH(Iter iter, const math::Aabb& bounding,
      const std::vector<Triangle>& triangles) {
    float cost, split;
    std::vector<Triangle> left_triangles, right_triangles;
    math::Aabb left_box, right_box;

    detail::findSplit(bounding, iter.axis(), triangles, cost, split,
        left_box, right_box, left_triangles, right_triangles);

    if (iter.depth() > 20 || cost > detail::COST_INTERSECT * triangles.size()) {
      iter.leaf(triangles);
    } else {
      iter.split(split);
      buildTreeSAH(iter.left(), left_box, left_triangles);
      buildTreeSAH(iter.right(), right_box, right_triangles);
    }
  }
}

#endif /* end of include guard: SAH_HPP_ID3KV0HO */
