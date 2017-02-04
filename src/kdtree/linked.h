#ifndef KDTREE_LINKED_H_
#define KDTREE_LINKED_H_

#include <vector>

#include "geometry/triangle.h"
#include "kdtree/util.h"

namespace kdtree {
struct KdNodeLinked {
  KdNodeLinked(Axis axis,
               float distance,
               KdNodeLinked* left,
               KdNodeLinked* right)
      : axis(axis),
        distance(distance),
        triangles(nullptr),
        left(left),
        right(right) {}

  KdNodeLinked(std::vector<const geometry::Triangle*>* triangles)
      : axis(),
        distance(),
        triangles(triangles),
        left(nullptr),
        right(nullptr) {}

  ~KdNodeLinked() {
    delete left;
    delete right;
    delete triangles;
  }

  Axis axis;
  float distance;
  std::vector<const geometry::Triangle*>* triangles;

  KdNodeLinked* left;
  KdNodeLinked* right;
};
}  // namespace kdtree

#endif  // KDTREE_LINKED_H_
