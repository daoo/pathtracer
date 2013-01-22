#ifndef TRAVERSE_HPP_IJFE6LBZ
#define TRAVERSE_HPP_IJFE6LBZ

#include "math/ray.hpp"
#include "trace/intersection.hpp"
#include "trace/kdtree/array.hpp"
#include "trace/triangle.hpp"

namespace trace
{
  namespace kdtree
  {
    inline bool searchTree(const KdTreeArray& tree, math::Ray& ray, Intersection& isect)
    {
      const float initial_maxt = ray.maxt;

      float mint = ray.mint;
      float maxt = ray.maxt;

      unsigned int index = 0;
      Axis axis = X;

      while (true) {
        assert(index < tree.nodes.size());

        const KdTreeArray::Node& node = tree.nodes[index];

        if (isLeaf(node)) {
          bool hit = false;
          if (hasTriangles(node)) {
            for (const Triangle& tri : getTriangles(tree, node)) {
              hit |= intersects(tri, ray, isect);
            }
          }

          if (hit && ray.maxt < maxt) {
            return true;
          } else if (maxt == initial_maxt) {
            return false;
          } else {
            index = 0;
            axis  = X;

            mint = maxt;
            maxt = initial_maxt;
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

#endif /* end of include guard: TRAVERSE_HPP_IJFE6LBZ */
