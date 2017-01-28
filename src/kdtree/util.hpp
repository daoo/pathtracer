#ifndef UTIL_HPP_26AQ5WXE
#define UTIL_HPP_26AQ5WXE

#include <glm/glm.hpp>

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
}

#endif /* end of include guard: UTIL_HPP_26AQ5WXE */
