#ifndef TREE_HPP_47RCBESP
#define TREE_HPP_47RCBESP

#include "kdtree/build/halfsplits.hpp"
#include "kdtree/dt/linked.hpp"
#include "kdtree/traverse/restart.hpp"
#include "math/ray.hpp"

namespace kdtree {
  typedef KdTreeLinked KdTree;

  template <typename Tree>
  void buildTree(Tree& iter, const std::vector<Triangle>& triangles) {
    build::halfSplitsBuildTree(typename Tree::BuildIter(iter),
        helpers::findBounding(triangles), triangles);
  }

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

#endif /* end of include guard: TREE_HPP_47RCBESP */
