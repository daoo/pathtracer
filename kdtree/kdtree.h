#ifndef KDTREE_KDTREE_H_
#define KDTREE_KDTREE_H_

#include <optional>
#include <vector>

#include "geometry/aap.h"
#include "geometry/triangle.h"

namespace geometry {
struct Ray;
struct TriRayIntersection;
}  // namespace geometry

namespace kdtree {
class KdNode {
 public:
  KdNode(geometry::Aap plane, KdNode* left, KdNode* right)
      : plane_(plane), triangles_(nullptr), left_(left), right_(right) {}

  explicit KdNode(std::vector<const geometry::Triangle*>* triangles)
      : plane_(geometry::X, 0),
        triangles_(triangles),
        left_(nullptr),
        right_(nullptr) {}

  ~KdNode() {
    delete left_;
    delete right_;
    delete triangles_;
  }

  inline geometry::Aap GetPlane() const { return plane_; }
  inline std::vector<const geometry::Triangle*>* GetTriangles() const {
    return triangles_;
  }

  inline const KdNode* GetLeft() const { return left_; }
  inline const KdNode* GetRight() const { return right_; }

 private:
  geometry::Aap plane_;
  std::vector<const geometry::Triangle*>* triangles_;

  KdNode* left_;
  KdNode* right_;
};

class KdTree {
 public:
  explicit KdTree(const KdNode* root) : root_(root) {}

  KdTree(KdTree&& other) : root_(other.root_) { other.root_ = nullptr; }

  ~KdTree() { delete root_; }

  const KdNode* GetRoot() const { return root_; }

 private:
  const KdNode* root_;
};

std::optional<geometry::TriRayIntersection> search_tree(
    const KdTree& tree,
    const geometry::Ray& ray,
    float tmin,
    float tmax);
}  // namespace kdtree

#endif  // KDTREE_KDTREE_H_
