#ifndef RAY_HPP_CNODB0L7
#define RAY_HPP_CNODB0L7

#include <glm/glm.hpp>

namespace math {
  class Ray {
    public:
      Ray() : mint(0.0f), maxt(FLT_MAX) {}

      Ray(const glm::vec3& origin, const glm::vec3& direction, float start = 0.0f, float end = FLT_MAX) :
        o(origin), d(direction), mint(start), maxt(end) {}

      glm::vec3 o, d;
      float mint, maxt;

      glm::vec3 operator()(float t) const {
        return o + d * t;
      }

      // In pbrt we have:
      // float time; // For motion blur
      // int depth;  // Number of bounces
  };
}

#endif /* end of include guard: RAY_HPP_CNODB0L7 */
