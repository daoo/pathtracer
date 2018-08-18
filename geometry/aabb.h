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

  inline Aabb Translate(glm::vec3 delta) const {
    return Aabb(center_ + delta, half_);
  }

  inline Aabb Enlarge(glm::vec3 delta) const {
    return Aabb(center_, half_ + delta);
  }

  static inline Aabb FromExtents(glm::vec3 min, glm::vec3 max) {
    glm::vec3 size = max - min;
    glm::vec3 half = size / 2.0f;
    return Aabb(min + half, half);
  }

  static inline Aabb Unit() {
    return Aabb(glm::vec3(0, 0, 0), glm::vec3(0.5, 0.5, 0.5));
  }

 private:
  glm::vec3 center_;
  glm::vec3 half_;
};
}  // namespace geometry

#endif  // GEOMETRY_AABB_H_
