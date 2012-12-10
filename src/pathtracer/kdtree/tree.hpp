#ifndef TREE_HPP_47RCBESP
#define TREE_HPP_47RCBESP

#include "math/ray.hpp"
#include "pathtracer/kdtree/array.hpp"
#include "pathtracer/kdtree/build.hpp"
#include "pathtracer/kdtree/linked.hpp"
#include "pathtracer/kdtree/sah.hpp"
#include "pathtracer/kdtree/traverse.hpp"

namespace kdtree {
#if defined(ARRAY_TREE)
  typedef KdTreeArray KdTree;
#elif defined(POINTER_TREE)
  typedef KdTreeLinked KdTree;
#else
  typedef KdTreeArray KdTree;
#endif

  template <typename Tree>
  void buildTree(Tree& tree, const std::vector<Triangle>& triangles) {
    buildTreeSAH(typename Tree::BuildIter(tree),
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
