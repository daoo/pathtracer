#include "aabb.hpp"

using namespace glm;
using namespace std;

namespace math {
  vec3 center(const Aabb& a) {
    return (a.min + a.max) * 0.5f;
  }

  vec3 half_size(const Aabb& a) {
    return (a.max - a.min) * 0.5f;
  }

  float volume(const Aabb& a) {
    vec3 d = a.max - a.min;
    return d.x * d.y * d.z;
  }

  float area(const Aabb& a) {
    vec3 d = a.max - a.min;
    return d.x*d.y*2.0f + d.x*d.z*2.0f + d.z*d.y*2.0f;
  }

  Aabb make_aabb(const vec3& min, const vec3& max) {
    return { min, max };
  }

  Aabb make_aabb(const vec3& position, const float radius) {
    return { position - radius, position + radius };
  }

  Aabb make_inverse_extreme_aabb() {
    return make_aabb(
        vec3(FLT_MAX, FLT_MAX, FLT_MAX),
        vec3(-FLT_MAX, -FLT_MAX, -FLT_MAX));
  }

  Aabb make_aabb(const vec3 *positions, const size_t numPositions) {
    Aabb result = make_inverse_extreme_aabb();

    for (size_t i = 0; i < numPositions; ++i) {
      result = combine(result, positions[i]);
    }

    return result;
  }

  Aabb combine(const Aabb& a, const Aabb& b) {
    return { min(a.min, b.min), max(a.max, b.max) };
  }

  Aabb combine(const Aabb& a, const vec3& pt) {
    return { min(a.min, pt), max(a.max, pt) };
  }
}
