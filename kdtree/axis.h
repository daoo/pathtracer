#ifndef KDTREE_AXIS_H_
#define KDTREE_AXIS_H_

#include <glm/glm.hpp>

#include "geometry/aap.h"

namespace kdtree {
// A look up table have been empirically proven to be the fastest way to
// calculate the next axis, compared to using modulo addition and bit hacks.
constexpr geometry::Axis NEXT[] = {geometry::Y, geometry::Z, geometry::X};
constexpr inline geometry::Axis next_axis(geometry::Axis axis) {
  return NEXT[axis];
}
}  // namespace kdtree

#endif  // KDTREE_AXIS_H_
