#include "kdtree/util.h"

#include <assert.h>
#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include "geometry/aabb.h"
#include "geometry/split.h"
#include "geometry/triangle.h"
#include "geometry/tribox.h"

using geometry::Aabb;
using geometry::AabbSplit;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using std::vector;

namespace kdtree {
namespace {
struct IntersectResults {
  vector<const Triangle*> left;
  vector<const Triangle*> right;
};

IntersectResults intersect_test(const vector<const Triangle*>& triangles,
                                const Aabb& left_aabb,
                                const Aabb& right_aabb) {
  IntersectResults results;
  results.left.reserve(triangles.size());
  results.right.reserve(triangles.size());
  for (const Triangle* triangle : triangles) {
    bool in_left =
        tri_box_overlap(left_aabb, triangle->v0, triangle->v1, triangle->v2);
    bool in_right =
        tri_box_overlap(right_aabb, triangle->v0, triangle->v1, triangle->v2);
    assert(in_left || in_right);
    if (in_left) results.left.emplace_back(triangle);
    if (in_right) results.right.emplace_back(triangle);
  }

  results.left.shrink_to_fit();
  results.right.shrink_to_fit();
  return results;
}
}  // namespace

Split split_box(const Box& parent, const Aap& plane) {
  AabbSplit aabbs = split(parent.boundary, plane);
  glm::vec3 delta(0, 0, 0);
  delta[plane.GetAxis()] = glm::epsilon<float>();
  Aabb left_aabb = aabbs.left.Translate(-delta).Enlarge(delta);
  Aabb right_aabb = aabbs.right.Translate(delta).Enlarge(delta);
  IntersectResults triangles =
      intersect_test(parent.triangles, left_aabb, right_aabb);
  Box left{left_aabb, triangles.left};
  Box right{right_aabb, triangles.right};
  return Split{plane, left, right};
}
}  // namespace kdtree
