#ifndef AABB_HPP_VOTKJJ0Y
#define AABB_HPP_VOTKJJ0Y

#include <glm/glm.hpp>
#include <ostream>

namespace math
{
  struct Aabb
  {
    glm::vec3 center;
    glm::vec3 half;
  };

  inline float surfaceArea(const Aabb& box)
  {
    return 8.0f * (
        box.half.x * box.half.y +
        box.half.x * box.half.z +
        box.half.y * box.half.z);
  }

  inline std::ostream& operator<<(std::ostream& stream, const Aabb& aabb)
  {
    stream << "(" << aabb.center.x << ", " << aabb.center.y << ", "
           << aabb.center.z << ")" << " " << "(" << aabb.half.x << ", "
           << aabb.half.y << ", " << aabb.half.z << ")";

    return stream;
  }
}

#endif /* end of include guard: AABB_HPP_VOTKJJ0Y */
