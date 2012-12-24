#ifndef TREE_HPP_47RCBESP
#define TREE_HPP_47RCBESP

#include "math/ray.hpp"
#include "trace/kdtree/array.hpp"
#include "trace/kdtree/traverse.hpp"

namespace trace
{
  namespace kdtree
  {
    typedef KdTreeArray KdTree;

    void buildTree(KdTree& tree, const std::vector<Triangle>& triangles);

    inline bool intersects(const KdTree& tree, math::Ray& ray, Intersection& isect)
    {
      return searchTree(tree, ray, isect);
    }

    inline bool intersects(const KdTree& tree, const math::Ray& ray)
    {
      Intersection isect;
      math::Ray raycopy(ray);
      return searchTree(tree, raycopy, isect);
    }
  }
}

#endif /* end of include guard: TREE_HPP_47RCBESP */
