#ifndef KDTREE_UTIL_H_
#define KDTREE_UTIL_H_

#include <vector>

#include "geometry/aabb.h"
#include "geometry/aap.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

namespace kdtree {
// A look up table have been empirically proven to be the fastest way to
// calculate the next axis, compared to using modulo addition and bit hacks.
constexpr geometry::Axis NEXT[] = {geometry::Y, geometry::Z, geometry::X};
constexpr inline geometry::Axis next_axis(geometry::Axis axis) {
  return NEXT[axis];
}

static_assert(next_axis(geometry::X) == geometry::Y, "incorrect next");
static_assert(next_axis(geometry::Y) == geometry::Z, "incorrect next");
static_assert(next_axis(geometry::Z) == geometry::X, "incorrect next");

struct Box {
  geometry::Aabb boundary;
  std::vector<const geometry::Triangle*> triangles;
};

struct Split {
  geometry::Aap plane;
  Box left, right;
};

Split split_box(const Box& parent, const geometry::Aap& plane);
}  // namespace kdtree

#endif  // KDTREE_UTIL_H_
