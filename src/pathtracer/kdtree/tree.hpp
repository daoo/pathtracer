#ifndef TREE_HPP_47RCBESP
#define TREE_HPP_47RCBESP

#include "math/ray.hpp"
#include "pathtracer/kdtree/array.hpp"

namespace kdtree {
  typedef KdTreeArray KdTree;

  void buildTree(KdTree& tree, const std::vector<Triangle>& triangles);

  bool intersects(const KdTree& tree, math::Ray& ray, Intersection& isect);
  bool intersects(const KdTree& tree, const math::Ray& ray);
}

#endif /* end of include guard: TREE_HPP_47RCBESP */
