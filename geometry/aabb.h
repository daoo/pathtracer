#ifndef GEOMETRY_AABB_H_
#define GEOMETRY_AABB_H_

#include <glm/glm.hpp>

namespace geometry {
class Aabb {
 public:
  Aabb(glm::vec3 center, glm::vec3 half) : center_(center), half_(half) {}

  inline glm::vec3 GetMin() const { return center_ - half_; }
  inline glm::vec3 GetMax() const { return center_ + half_; }
  inline glm::vec3 GetCenter() const { return center_; }
  inline glm::vec3 GetHalf() const { return half_; }

  inline float GetSurfaceArea() const {
    return 8.0f * (half_.x * half_.y + half_.x * half_.z + half_.y * half_.z);
  }

 private:
  glm::vec3 center_;
  glm::vec3 half_;
};
}  // namespace geometry

#endif  // GEOMETRY_AABB_H_
