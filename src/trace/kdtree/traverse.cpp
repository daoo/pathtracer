#include "traverse.hpp"

#include "trace/geometry/triangle.hpp"
#include "trace/geometry/triray.hpp"

using namespace glm;
using namespace std;
using namespace trace;
using namespace trace::kdtree;

namespace
{
  bool intersect_triangles(
      const vector<Triangle>& triangles,
      const Ray& ray,
      float mint,
      float& maxt,
      vec3& normal,
      const Material*& material)
  {
    bool hit = false;

    for (const Triangle& triangle : triangles) {
      float t;
      vec3 n;
      if (triray(triangle, ray, t, n)) {
        if (t >= mint && t <= maxt) {
          normal   = n;
          material = triangle.material;

          maxt = t;

          hit = true;
        }
      }
    }

    return hit;
  }

  struct TreeSearch
  {
    const KdTreeArray& tree;
    const Ray& ray;
    float tmaxinit;
  };

  bool go(
      const TreeSearch& search,
      float raymaxt,
      float tmin,
      float tmax,
      int index,
      Axis axis,
      Intersection& isect)
  {
    ArrayNode node = search.tree.get_node(index);

    if (node.is_leaf()) {
      bool hit = false;
      if (node.has_triangles()) {
        hit = intersect_triangles(
            search.tree.get_triangles(node),
            search.ray,
            tmin,
            raymaxt,
            isect.normal,
            isect.material);
      }

      if (hit && raymaxt < tmax) {
        isect.position = search.ray.param(raymaxt);

        return true;
      } else if (tmax == search.tmaxinit) {
        return false;
      } else {
        return go(
            search,
            raymaxt,
            tmax,
            search.tmaxinit,
            0, X,
            isect);
      }
    } else {
      const float p = node.get_split();
      const float o = search.ray.origin[axis];
      const float d = search.ray.direction[axis];

      float t = (p - o) / d;

      unsigned int first  = KdTreeArray::left_child(index);
      unsigned int second = KdTreeArray::right_child(index);

      if (d < 0) {
        swap(first, second);
      }

      if (t >= tmax) {
        return go(
            search,
            raymaxt, tmin, tmax,
            first, next_axis(axis),
            isect);
      } else if (t <= tmin) {
        return go(
            search,
            raymaxt, tmin, tmax,
            second, next_axis(axis),
            isect);
      } else {
        return go(
            search,
            raymaxt, tmin, t,
            first, next_axis(axis),
            isect);
      }
    }
  }
}

namespace trace
{
  namespace kdtree
  {
    bool search_tree(
        const KdTreeArray& tree,
        const Ray& ray,
        float tmininit,
        float tmaxinit,
        Intersection& isect)
    {
      return go(
          { tree, ray, tmaxinit },
          tmaxinit, tmininit, tmaxinit,
          0, X,
          isect);
    }
  }
}
