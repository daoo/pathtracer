#ifndef KDTREE_LINKED_H_
#define KDTREE_LINKED_H_

#include <vector>

#include "geometry/triangle.h"
#include "kdtree/util.h"

namespace kdtree {
class KdNodeLinked {
 public:
  KdNodeLinked(Axis axis,
               float distance,
               KdNodeLinked* left,
               KdNodeLinked* right)
      : axis_(axis),
        distance_(distance),
        triangles_(nullptr),
        left_(left),
        right_(right) {}

  explicit KdNodeLinked(std::vector<const geometry::Triangle*>* triangles)
      : axis_(),
        distance_(),
        triangles_(triangles),
        left_(nullptr),
        right_(nullptr) {}

  ~KdNodeLinked() {
    delete left_;
    delete right_;
    delete triangles_;
  }

  inline Axis GetAxis() const { return axis_; }
  inline float GetDistance() const { return distance_; }
  inline std::vector<const geometry::Triangle*>* GetTriangles() const {
    return triangles_;
  }

  inline const KdNodeLinked* GetLeft() const { return left_; }
  inline const KdNodeLinked* GetRight() const { return right_; }

 private:
  Axis axis_;
  float distance_;
  std::vector<const geometry::Triangle*>* triangles_;

  KdNodeLinked* left_;
  KdNodeLinked* right_;
};

class KdTreeLinked {
 public:
  KdTreeLinked(const KdNodeLinked* root,
               const std::vector<geometry::Triangle>& triangles)
      : root_(root), triangles_(triangles) {}

  KdTreeLinked(KdTreeLinked&& other)
      : root_(other.root_), triangles_(std::move(other.triangles_)) {
    other.root_ = nullptr;
  }

  ~KdTreeLinked() { delete root_; }

  const KdNodeLinked* GetRoot() const { return root_; }

 private:
  const KdNodeLinked* root_;
  const std::vector<geometry::Triangle>& triangles_;
};
}  // namespace kdtree

#endif  // KDTREE_LINKED_H_
