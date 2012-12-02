#ifndef COLLISIONS_HPP_PBJPV29H
#define COLLISIONS_HPP_PBJPV29H

#include "math/aabb.hpp"
#include "math/ray.hpp"

namespace math {
  bool intersect(const Aabb&, const Ray&, float&, float&);
  bool overlaps(const Aabb&, const Aabb&);

  inline bool overlaps(const Aabb& a, const Aabb& b) {
    return a.max.x > b.min.x && a.min.x < b.max.x
        && a.max.y > b.min.y && a.min.y < b.max.y
        && a.max.z > b.min.z && a.min.z < b.max.z;
  }

  inline bool intersect(const Aabb& a, const Ray& r, float& hit0, float& hit1) {
    float t0 = r.mint, t1 = r.maxt;
    for (int i = 0; i < 3; ++i) {
      float invRayDir = 1.0f / r.d[i];
      float tNear     = (a.min[i] - r.o[i]) * invRayDir;
      float tFar      = (a.max[i] - r.o[i]) * invRayDir;

      if (tNear > tFar) {
        std::swap(tNear, tFar);
      }

      t0 = tNear > t0 ? tNear : t0;
      t1 = tFar < t1 ? tFar : t1;
      if (t0 > t1)
        return false;
    }

    hit0 = t0;
    hit1 = t1;
    return true;
  }
}

#endif /* end of include guard: COLLISIONS_HPP_PBJPV29H */
