#include "trace/kdtree/tree.hpp"

#include "trace/kdtree/build.hpp"
#include "trace/kdtree/linked.hpp"
#include "trace/kdtree/optimize.hpp"

namespace trace
{
  namespace kdtree
  {
    void buildTree(KdTree& tree, const std::vector<Triangle>& triangles)
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
  }
}
