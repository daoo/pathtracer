#include "kdtree/array.h"

#include <glm/glm.hpp>

#include <algorithm>

#include "geometry/ray.h"
#include "geometry/triray.h"
#include "kdtree/util.h"

using geometry::Axis;
using std::optional;

namespace kdtree {
optional<geometry::TriRayIntersection> search_tree(const KdTreeArray& tree,
                                                   const geometry::Ray& ray,
                                                   float tmininit,
                                                   float tmaxinit) {
  unsigned int index = 0;
  float tmin = tmininit;
  float tmax = tmaxinit;
  Axis axis = geometry::X;

  while (true) {
    KdNodeArray node = tree.GetNode(index);

    if (node.IsLeaf()) {
      optional<geometry::TriRayIntersection> result =
          find_closest(tree.GetTriangles(node), ray, tmin, tmax);
      if (result) {
        return result;
      } else if (tmax == tmaxinit) {
        return optional<geometry::TriRayIntersection>();
      } else {
        tmin = tmax;
        tmax = tmaxinit;
        index = 0;
        axis = geometry::X;
      }
    } else {
      float p = node.GetSplit();
      float o = ray.origin[axis];
      float d = ray.direction[axis];
      float t = (p - o) / d;
      unsigned int first = KdTreeArray::LeftChild(index);
      unsigned int second = KdTreeArray::RightChild(index);

      if (d < 0) {
        std::swap(first, second);
      }

      if (t >= tmax) {
        index = first;
        axis = next_axis(axis);
      } else if (t <= tmin) {
        index = second;
        axis = next_axis(axis);
      } else {
        index = first;
        axis = next_axis(axis);
        tmax = t;
      }
    }
  }
}
}  // namespace kdtree
