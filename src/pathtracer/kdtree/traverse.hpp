#ifndef TRAVERSE_HPP_IJFE6LBZ
#define TRAVERSE_HPP_IJFE6LBZ

#include "math/ray.hpp"
#include "pathtracer/intersection.hpp"
#include "pathtracer/kdtree/array.hpp"

namespace kdtree {
  inline bool searchTree(const KdTreeArray& tree, math::Ray& ray, Intersection& isect) {
    const float initial_maxt = ray.maxt;

    float mint = ray.mint;
    float maxt = ray.maxt;

    size_t index = 0;
    Axis axis = X;

    while (true) {
      const KdTreeArray::Node* node = &tree.m_nodes[index];

      if (node->isLeaf()) {
        bool hit = false;
        if (node->hasTriangles()) {
          for (const Triangle& tri : tree.m_leaf_store[node->getIndex()]) {
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

          mint  = maxt;
          maxt  = initial_maxt;
        }
      } else if (node->isSplit()) {
        float p = node->getDistance();

        float o = swizzle(ray.origin, axis);
        float d = swizzle(ray.direction, axis);

        float t = (p - o) / d;

        size_t first = KdTreeArray::leftChild(index);
        size_t second = KdTreeArray::rightChild(index);
        order(d, first, second);

        if (t >= maxt) {
          index = first;
        } else if (t <= mint) {
          index = second;
        } else {
          index = first;
          maxt  = t;
        }

        axis = KdTreeArray::next(axis);
      }
    }

    assert(false && "If this happens, something went very wrong.");
    return false;
  }
}

#endif /* end of include guard: TRAVERSE_HPP_IJFE6LBZ */
