#ifndef KDTREE_LINKED_H_
#define KDTREE_LINKED_H_

#include <optional>
#include <vector>

#include "geometry/triangle.h"
#include "kdtree/util.h"

namespace geometry {
struct Ray;
struct TriRayIntersection;
}  // namespace geometry

namespace kdtree {
class KdNodeLinked {
 public:
  KdNodeLinked(geometry::Aap plane, KdNodeLinked* left, KdNodeLinked* right)
      : plane_(plane), triangles_(nullptr), left_(left), right_(right) {}

  explicit KdNodeLinked(std::vector<const geometry::Triangle*>* triangles)
      : plane_(geometry::X, 0),
        triangles_(triangles),
        left_(nullptr),
        right_(nullptr) {}

  ~KdNodeLinked() {
    delete left_;
    delete right_;
    delete triangles_;
  }

  inline geometry::Aap GetPlane() const { return plane_; }
  inline std::vector<const geometry::Triangle*>* GetTriangles() const {
    return triangles_;
  }

  inline const KdNodeLinked* GetLeft() const { return left_; }
  inline const KdNodeLinked* GetRight() const { return right_; }

 private:
  geometry::Aap plane_;
  std::vector<const geometry::Triangle*>* triangles_;

  KdNodeLinked* left_;
  KdNodeLinked* right_;
};

class KdTreeLinked {
 public:
  KdTreeLinked(const KdNodeLinked* root) : root_(root) {}

  KdTreeLinked(KdTreeLinked&& other) : root_(other.root_) {
    other.root_ = nullptr;
  }

  ~KdTreeLinked() { delete root_; }

  const KdNodeLinked* GetRoot() const { return root_; }

 private:
  const KdNodeLinked* root_;
};

std::optional<geometry::TriRayIntersection> search_tree(
    const KdTreeLinked& tree,
    const geometry::Ray& ray,
    float tmin,
    float tmax);
}  // namespace kdtree

#endif  // KDTREE_LINKED_H_
