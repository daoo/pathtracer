#include "kdtree/sah_fast.h"

#include "kdtree/sah_fast_impl.h"

using geometry::Triangle;
using std::vector;

namespace kdtree {
KdTreeLinked build(const vector<Triangle>& triangles) {
  vector<const Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  return KdTreeLinked(
      BuildHelper(0, KdBox{geometry::find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
