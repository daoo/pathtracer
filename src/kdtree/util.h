#ifndef KDTREE_UTIL_H_
#define KDTREE_UTIL_H_

#include <vector>

#include "geometry/aabb.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

namespace kdtree {
enum Axis { X = 0, Y = 1, Z = 2 };

// A look up table have been empirically proven to be the fastest way to
// calculate the next axis, compared to using modulo addition and bit hacks.
constexpr Axis NEXT[] = {Y, Z, X};
constexpr inline Axis next_axis(Axis axis) {
  return NEXT[axis];
}

static_assert(next_axis(X) == Y, "incorrect next");
static_assert(next_axis(Y) == Z, "incorrect next");
static_assert(next_axis(Z) == X, "incorrect next");

struct Box {
  geometry::Aabb boundary;
  std::vector<const geometry::Triangle*> triangles;
};

struct Split {
  Axis axis;
  float distance;
  Box left, right;
};

Split split_box(const Box& parent, Axis axis, float distance);
}  // namespace kdtree

#endif  // KDTREE_UTIL_H_
