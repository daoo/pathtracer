#include "tree.hpp"

#include "pathtracer/kdtree/build.hpp"
#include "pathtracer/kdtree/linked.hpp"
#include "pathtracer/kdtree/optimize.hpp"
#include "pathtracer/kdtree/traverse.hpp"

namespace kdtree {
  void buildTree(KdTree& tree, const std::vector<Triangle>& triangles) {
    std::vector<const Triangle*> ptrs;
    for (const Triangle& tri : triangles) {
      ptrs.push_back(&tri);
    }

    KdTreeLinked tmp;
    tmp.m_root = new KdTreeLinked::Node;
    buildTreeNaive(tmp.m_root, 0, X, helpers::findBounding(triangles), ptrs);

    optimize(tree, tmp);
  }

  bool intersects(const KdTree& tree, math::Ray& ray, Intersection& isect) {
    return searchTree(tree, ray, isect);
  }

  bool intersects(const KdTree& tree, const math::Ray& ray) {
    Intersection isect;
    math::Ray raycopy(ray);
    return searchTree(tree, raycopy, isect);
  }
}
