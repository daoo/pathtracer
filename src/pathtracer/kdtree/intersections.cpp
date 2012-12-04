#include "intersections.hpp"

#include "kdtree/traverse/restart.hpp"

namespace kdtree {
  bool intersects(const KdTreeLinked& tree, math::Ray& ray, Intersection& isect) {
    return traverse::restartSearchTree<KdTreeLinked>(tree, ray, isect);
  }

  bool intersects(const KdTreeLinked& tree, const math::Ray& ray) {
    Intersection isect;
    math::Ray raycopy(ray);
    return traverse::restartSearchTree<KdTreeLinked>(tree, raycopy, isect);
  }
}
