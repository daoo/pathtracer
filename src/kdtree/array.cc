#include "kdtree/array.h"

#include <glm/glm.hpp>

#include <algorithm>

#include "geometry/ray.h"
#include "geometry/triray.h"
#include "kdtree/util.h"

using std::experimental::optional;

namespace kdtree {
optional<geometry::TriRayIntersection> search_tree(const KdTreeArray& tree,
                                                   const geometry::Ray& ray,
                                                   float tmininit,
                                                   float tmaxinit) {
  unsigned int index = 0;
  float tmin = tmininit;
  float tmax = tmaxinit;
  Axis axis = X;

  while (true) {
    KdNodeArray node = tree.get_node(index);

    if (node.is_leaf()) {
      optional<geometry::TriRayIntersection> result =
          find_closest(tree.get_triangles(node), ray, tmin, tmax);
      if (result) {
        return result;
      } else if (tmax == tmaxinit) {
        return optional<geometry::TriRayIntersection>();
      } else {
        tmin = tmax;
        tmax = tmaxinit;
        index = 0;
        axis = X;
      }
    } else {
      float p = node.get_split();
      float o = ray.origin[axis];
      float d = ray.direction[axis];
      float t = (p - o) / d;
      unsigned int first = KdTreeArray::left_child(index);
      unsigned int second = KdTreeArray::right_child(index);

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
