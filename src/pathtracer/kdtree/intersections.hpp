#ifndef INTERSECTIONS_HPP_HYFVF302
#define INTERSECTIONS_HPP_HYFVF302

#include "kdtree/traverse/stack.hpp"
#include "kdtree/traverse/restart.hpp"
#include "math/ray.hpp"

namespace kdtree {
  bool intersects(const KdTreeLinked& tree, math::Ray& ray, Intersection& isect) {
    return traverse::restart::searchTree(tree, ray, isect);
  }

  bool intersects(const KdTreeLinked& tree, const math::Ray& ray) {
    Intersection isect;
    math::Ray raycopy(ray);
    return traverse::restart::searchTree(tree, raycopy, isect);
  }
}

#endif /* end of include guard: INTERSECTIONS_HPP_HYFVF302 */
