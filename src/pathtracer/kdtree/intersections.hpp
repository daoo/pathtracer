#ifndef INTERSECTIONS_HPP_LOZBUIUH
#define INTERSECTIONS_HPP_LOZBUIUH

#include "kdtree/traverse/restart.hpp"
#include "math/ray.hpp"

namespace kdtree {
  template <typename Tree>
  bool intersects(const Tree& tree, math::Ray& ray, Intersection& isect) {
    return traverse::restartSearchTree<Tree>(tree, ray, isect);
  }

  template <typename Tree>
  bool intersects(const Tree& tree, const math::Ray& ray) {
    Intersection isect;
    math::Ray raycopy(ray);
    return traverse::restartSearchTree<Tree>(tree, raycopy, isect);
  }
}

#endif /* end of include guard: INTERSECTIONS_HPP_LOZBUIUH */
