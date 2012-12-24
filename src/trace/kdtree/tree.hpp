#ifndef TREE_HPP_47RCBESP
#define TREE_HPP_47RCBESP

#include "math/ray.hpp"
#include "trace/kdtree/array.hpp"
#include "trace/kdtree/build.hpp"
#include "trace/kdtree/linked.hpp"
#include "trace/kdtree/optimize.hpp"
#include "trace/kdtree/traverse.hpp"

namespace trace
{
  namespace kdtree
  {
    typedef KdTreeArray KdTree;

    inline void buildTree(KdTree& tree, const std::vector<Triangle>& triangles)
    {
      std::vector<const Triangle*> ptrs;
      for (const Triangle& tri : triangles) {
        ptrs.push_back(&tri);
      }

      KdTreeLinked tmp;
      tmp.root = new KdTreeLinked::Node;
      buildTreeSAH(tmp.root, 0, X, findBounding(triangles), ptrs);

      optimize(tree, tmp);
    }

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
