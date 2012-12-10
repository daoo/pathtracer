#ifndef TREE_HPP_47RCBESP
#define TREE_HPP_47RCBESP

#include "math/ray.hpp"
#include "pathtracer/kdtree/array.hpp"
#include "pathtracer/kdtree/build.hpp"
#include "pathtracer/kdtree/linked.hpp"
#include "pathtracer/kdtree/traverse.hpp"

namespace kdtree {
  typedef KdTreeLinked KdTree;

  inline void buildTree(KdTreeLinked& tree, const std::vector<Triangle>& triangles) {
    std::vector<const Triangle*> ptrs;
    for (const Triangle& tri : triangles) {
      ptrs.push_back(&tri);
    }

    tree.m_root = new KdTreeLinked::Node;
    buildTreeNaive(tree.m_root, 0, X, helpers::findBounding(triangles), ptrs);
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
