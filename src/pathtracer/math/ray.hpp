#ifndef RAY_HPP_CNODB0L7
#define RAY_HPP_CNODB0L7

#include <glm/glm.hpp>

namespace math {
  class Ray {
    public:
      Ray(const glm::vec3& o, const glm::vec3& d, float start, float end) :
        origin(o), direction(d), mint(start), maxt(end) { }

      glm::vec3 origin, direction;
      float mint, maxt;

      glm::vec3 operator()(float t) const {
        return origin + direction * t;
      }
  };
}

#endif /* end of include guard: RAY_HPP_CNODB0L7 */
