#ifndef TREE_HPP_47RCBESP
#define TREE_HPP_47RCBESP

#include "pathtracer/kdtree/build.hpp"
#include "pathtracer/kdtree/dt/array.hpp"
#include "pathtracer/kdtree/dt/linked.hpp"
#include "pathtracer/kdtree/traverse.hpp"
#include "pathtracer/math/ray.hpp"

namespace kdtree {
  typedef KdTreeArray KdTree;

  template <typename Tree>
  void buildTree(Tree& tree, const std::vector<Triangle>& triangles) {
    buildTree(typename Tree::BuildIter(tree),
        helpers::findBounding(triangles), triangles);
  }

  template <typename Tree>
  bool intersects(const Tree& tree, math::Ray& ray, Intersection& isect) {
    return searchTree<Tree>(tree, ray, isect);
  }

  template <typename Tree>
  bool intersects(const Tree& tree, const math::Ray& ray) {
    Intersection isect;
    math::Ray raycopy(ray);
    return searchTree<Tree>(tree, raycopy, isect);
  }
}

#endif /* end of include guard: TREE_HPP_47RCBESP */
