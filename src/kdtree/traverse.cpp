#include "kdtree/traverse.hpp"

#include "geometry/ray.hpp"
#include "geometry/triangle.hpp"
#include "geometry/triray.hpp"
#include "kdtree/array.hpp"
#include "kdtree/intersection.hpp"
#include "kdtree/util.hpp"
#include <algorithm>
#include <glm/glm.hpp>
#include <vector>

using namespace glm;
using namespace std;

namespace kdtree {
namespace {
bool intersect_triangles(const vector<geometry::Triangle>& triangles,
                         const geometry::Ray& ray,
                         float mint,
                         float& maxt,
                         vec3& normal,
                         const void*& tag) {
  bool hit = false;

  for (const geometry::Triangle& triangle : triangles) {
    float t;
    vec3 n;
    if (triray(triangle, ray, t, n)) {
      if (t >= mint && t <= maxt) {
        normal = n;
        tag = triangle.tag;

        maxt = t;

        hit = true;
      }
    }
  }

  return hit;
}
}

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
    ArrayNode node = tree.get_node(index);

    if (node.is_leaf()) {
      bool hit = intersect_triangles(tree.get_triangles(node), ray, tmin,
                                     raymaxt, isect.normal, isect.tag);

      if (hit && raymaxt < tmax) {
        isect.position = ray.param(raymaxt);
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
        swap(first, second);
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
}
