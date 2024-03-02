#include "kdtree/kdtree.h"

#include <glm/glm.hpp>

#include <utility>

#include "geometry/ray.h"
#include "geometry/triray.h"

using std::optional;

namespace kdtree {
optional<geometry::TriRayIntersection> search_tree(const KdTree& tree,
                                                   const geometry::Ray& ray,
                                                   float tmin,
                                                   float tmax) {
  const KdNode* node = tree.GetRoot();
  float t1 = tmin;
  float t2 = tmax;

  while (true) {
    if (node->GetTriangles() != nullptr) {
      optional<geometry::TriRayIntersection> result =
          find_closest(*node->GetTriangles(), ray, t1, t2);
      if (result) {
        return result;
      } else if (t2 == tmax) {
        return optional<geometry::TriRayIntersection>();
      } else {
        t1 = t2;
        t2 = tmax;
        node = tree.GetRoot();
      }
    } else {
      float p = node->GetPlane().GetDistance();
      float o = ray.origin[node->GetPlane().GetAxis()];
      float d = ray.direction[node->GetPlane().GetAxis()];
      const KdNode* first = node->GetLeft();
      const KdNode* second = node->GetRight();

      if (d < 0) {
        std::swap(first, second);
      }

      float t = (p - o) / d;
      if (t >= t2) {
        node = first;
      } else if (t <= t1) {
        node = second;
      } else {
        node = first;
        t2 = t;
      }
    }
  }
}
}  // namespace kdtree
