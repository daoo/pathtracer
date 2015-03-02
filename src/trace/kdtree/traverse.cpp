#include "traverse.hpp"
#include "trace/triangle.hpp"

using namespace glm;
using namespace math;
using namespace std;
using namespace trace;
using namespace trace::kdtree;

namespace
{
  bool intersectTriangles(
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
      if (intersects(triangle, ray, t, n)) {
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
      TreeSearch search,
      float raymaxt,
      float tmin,
      float tmax,
      int index,
      Axis axis,
      Intersection& isect)
  {
    assert(index < search.tree.nodes.size());

    const KdTreeArray::Node& node = search.tree.nodes[index];

    if (isLeaf(node)) {
      bool hit = intersectTriangles(
          getTriangles(search.tree, node),
          search.ray,
          tmin,
          raymaxt,
          isect.normal,
          isect.material);

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
      const float p = getSplit(node);
      const float o = search.ray.origin[axis];
      const float d = search.ray.direction[axis];

      float t = (p - o) / d;

      unsigned int first  = KdTreeArray::leftChild(index);
      unsigned int second = KdTreeArray::rightChild(index);

      if (d < 0) {
        std::swap(first, second);
      }

      if (t >= tmax) {
        return go(
            search,
            raymaxt, tmin, tmax,
            first, nextAxis(axis),
            isect);
      } else if (t <= tmin) {
        return go(
            search,
            raymaxt, tmin, tmax,
            second, nextAxis(axis),
            isect);
      } else {
        return go(
            search,
            raymaxt, tmin, t,
            first, nextAxis(axis),
            isect);
      }
    }
  }
}

namespace trace
{
  namespace kdtree
  {
    bool searchTree(
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
