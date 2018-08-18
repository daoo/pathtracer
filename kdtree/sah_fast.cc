#include "kdtree/sah_fast.h"

#include <vector>

#include "kdtree/sah_fast_impl.h"

using geometry::Triangle;
using std::vector;

namespace kdtree {
KdTree build(const vector<Triangle>& triangles) {
  vector<const Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  return KdTree(
      BuildHelper(0, KdBox{geometry::find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
