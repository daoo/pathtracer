#ifndef TREE_HPP_47RCBESP
#define TREE_HPP_47RCBESP

#include "trace/geometry/ray.hpp"
#include "trace/kdtree/array.hpp"
#include "trace/kdtree/traverse.hpp"

namespace trace
{
  namespace kdtree
  {
    typedef KdTreeArray KdTree;

    void buildTree(KdTree& tree, const std::vector<Triangle>& triangles);

    inline bool intersects(
        const KdTree& tree,
        const math::Ray& ray,
        float tmin,
        float tmax,
        Intersection& isect)
    {
      return searchTree(tree, ray, tmin, tmax, isect);
    }
  }
}

#endif /* end of include guard: TREE_HPP_47RCBESP */
