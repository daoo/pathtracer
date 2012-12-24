#ifndef UTIL_HPP_26AQ5WXE
#define UTIL_HPP_26AQ5WXE

#include <glm/glm.hpp>

namespace trace
{
  namespace kdtree
  {
    enum Axis
    {
      X = 0, Y = 1, Z = 2
    };

    // A look up table have been impirically proven to be the fastest way to
    // calculate the next axis, compared to using modulo addition and bit hacks.
    constexpr Axis NEXT[] = { Y, Z, X };
    constexpr inline Axis nextAxis(Axis axis)
    {
      return NEXT[axis];
    }

    static_assert(nextAxis(X) == Y, "incorrect next");
    static_assert(nextAxis(Y) == Z, "incorrect next");
    static_assert(nextAxis(Z) == X, "incorrect next");

    inline float swizzle(const glm::vec3& v, Axis c)
    {
      return v[c];
    }
  }
}

#endif /* end of include guard: UTIL_HPP_26AQ5WXE */
