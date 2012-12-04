#ifndef INTERSECTIONS_HPP_LOZBUIUH
#define INTERSECTIONS_HPP_LOZBUIUH

#include "kdtree/linked.hpp"
#include "math/ray.hpp"

namespace kdtree {
  bool intersects(const KdTreeLinked& tree, math::Ray& ray, Intersection& isect);
  bool intersects(const KdTreeLinked& tree, const math::Ray& ray);
}

#endif /* end of include guard: INTERSECTIONS_HPP_LOZBUIUH */
