#ifndef UTIL_HPP_26AQ5WXE
#define UTIL_HPP_26AQ5WXE

#include <glm/glm.hpp>

namespace kdtree {
  enum Axis {
    X = 0, Y = 1, Z = 2
  };

  inline Axis nextAxis(Axis axis) {
    return static_cast<Axis>((static_cast<size_t>(axis) + 1) % 3);
  }

  inline float swizzle(const glm::vec3& v, Axis c) {
    if (c == X)
      return v.x;
    else if (c == Y)
      return v.y;
    else
      return v.z;
  }
}

#endif /* end of include guard: UTIL_HPP_26AQ5WXE */
