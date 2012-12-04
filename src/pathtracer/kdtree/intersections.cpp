#include "intersections.hpp"

#include "kdtree/traverse/stack.hpp"

namespace kdtree {
  bool intersects(const KdTreeLinked& tree, math::Ray& ray, Intersection& isect) {
    return traverse::stackSearchTree<KdTreeLinked>(tree, ray, isect);
  }

  bool intersects(const KdTreeLinked& tree, const math::Ray& ray) {
    Intersection isect;
    math::Ray raycopy(ray);
    return traverse::stackSearchTree<KdTreeLinked>(tree, raycopy, isect);
  }
}
