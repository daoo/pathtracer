#include "trace/kdtree/tree.hpp"

#include "trace/geometry/bounding.hpp"
#include "trace/kdtree/build.hpp"
#include "trace/kdtree/linked.hpp"
#include "trace/kdtree/optimize.hpp"

#include <glm/glm.hpp>
#include <vector>

using namespace glm;
using namespace math;
using namespace std;

namespace trace
{
  namespace kdtree
  {
    void build_tree(KdTree& tree, const vector<Triangle>& triangles)
    {
      vector<const Triangle*> ptrs;
      for (const Triangle& tri : triangles) {
        ptrs.push_back(&tri);
      }

      KdTreeLinked tmp;
      tmp.root = new KdTreeLinked::Node;
      build_tree_sah(tmp.root, 0, X, find_bounding(triangles), ptrs);

      optimize(tree, tmp);
    }
  }
}
