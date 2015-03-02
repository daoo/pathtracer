#include "traverse.hpp"
#include "trace/triangle.hpp"

using namespace glm;
using namespace math;
using namespace std;
using namespace trace;

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
}

namespace trace
{
  namespace kdtree
  {
    bool searchTree(
        const KdTreeArray& tree,
        const Ray& ray,
        Intersection& isect)
    {
      float raymint = ray.mint;
      float raymaxt = ray.maxt;

      float mint = ray.mint;
      float maxt = ray.maxt;

      unsigned int index = 0;
      Axis axis = X;

      while (true) {
        assert(index < tree.nodes.size());

        const KdTreeArray::Node& node = tree.nodes[index];

        if (isLeaf(node)) {
          vec3 n;
          const Material* material(nullptr);
          bool hit = intersectTriangles(
              getTriangles(tree, node),
              ray,
              raymint,
              raymaxt,
              n,
              material);

          if (hit && raymaxt < maxt) {
            isect.position = ray(raymaxt);
            isect.normal   = n;
            isect.material = material;

            return true;
          } else if (maxt == ray.maxt) {
            return false;
          } else {
            index = 0;
            axis  = X;

            mint = maxt;
            maxt = ray.maxt;
          }
        } else {
          const float p = getSplit(node);
          const float o = ray.origin[axis];
          const float d = ray.direction[axis];

          float t = (p - o) / d;

          unsigned int first  = KdTreeArray::leftChild(index);
          unsigned int second = KdTreeArray::rightChild(index);

          if (d < 0) {
            std::swap(first, second);
          }

          if (t >= maxt) {
            axis  = nextAxis(axis);
            index = first;
          } else if (t <= mint) {
            axis  = nextAxis(axis);
            index = second;
          } else {
            axis  = nextAxis(axis);
            index = first;
            maxt  = t;
          }
        }
      }

      assert(false && "If this happens, something went very wrong.");
      return false;
    }
  }
}
