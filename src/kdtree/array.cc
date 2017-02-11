#include "kdtree/array.h"

#include <glm/glm.hpp>

#include <algorithm>
#include <vector>

#include "geometry/ray.h"
#include "geometry/triray.h"
#include "kdtree/intersection.h"
#include "kdtree/util.h"

using glm::vec3;
using std::vector;

namespace kdtree {
bool search_tree(const KdTreeArray& tree,
                 const geometry::Ray& ray,
                 const float tmininit,
                 const float tmaxinit,
                 Intersection& isect) {
  unsigned int index = 0;
  float raymaxt = tmaxinit;
  float tmin = tmininit;
  float tmax = tmaxinit;
  Axis axis = X;

  while (true) {
    KdNodeArray node = tree.get_node(index);

    if (node.is_leaf()) {
      optional<geometry::TriRayIntersection> result =
          find_closest(tree.get_triangles(node), ray, tmin, raymaxt);
      if (result) {
        raymaxt = result->t;
      }

      if (result && raymaxt < tmax) {
        isect.position = result->get_position();
        isect.normal = result->get_normal();
        isect.tag = result->triangle.tag;
        return true;
      } else if (tmax == tmaxinit) {
        return false;
      } else {
        tmin = tmax;
        tmax = tmaxinit;
        index = 0;
        axis = X;
      }
    } else {
      const float p = node.get_split();
      const float o = ray.origin[axis];
      const float d = ray.direction[axis];

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
