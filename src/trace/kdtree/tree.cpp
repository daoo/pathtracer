#include "trace/kdtree/tree.hpp"

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
    namespace
    {
      Aabb find_bounding(const vector<Triangle>& triangles)
      {
        vec3 min, max;

        for (const Triangle& tri : triangles) {
          min = glm::min(min, tri.v0);
          min = glm::min(min, tri.v1);
          min = glm::min(min, tri.v2);

          max = glm::max(max, tri.v0);
          max = glm::max(max, tri.v1);
          max = glm::max(max, tri.v2);
        }

        vec3 half = (max - min) / 2.0f;
        return { min + half, half };
      }
    }

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
