#include "kdtree/kdtree.h"

#include <glm/glm.hpp>

#include <utility>

#include "geometry/ray.h"
#include "geometry/triray.h"

using std::optional;

namespace kdtree {
optional<geometry::TriRayIntersection> search_tree(const KdTree& tree,
                                                   const geometry::Ray& ray,
                                                   float tmininit,
                                                   float tmaxinit) {
  const KdNode* node = tree.GetRoot();
  float tmin = tmininit;
  float tmax = tmaxinit;

  while (true) {
    if (node->GetTriangles() != nullptr) {
      optional<geometry::TriRayIntersection> result =
          find_closest(*node->GetTriangles(), ray, tmin, tmax);
      if (result) {
        return result;
      } else if (tmax == tmaxinit) {
        return optional<geometry::TriRayIntersection>();
      } else {
        tmin = tmax;
        tmax = tmaxinit;
        node = tree.GetRoot();
      }
    } else {
      float p = node->GetPlane().GetDistance();
      float o = ray.origin[node->GetPlane().GetAxis()];
      float d = ray.direction[node->GetPlane().GetAxis()];
      float t = (p - o) / d;
      const KdNode* first = node->GetLeft();
      const KdNode* second = node->GetRight();

      if (d < 0) {
        std::swap(first, second);
      }

      if (t >= tmax) {
        node = first;
      } else if (t <= tmin) {
        node = second;
      } else {
        node = first;
        tmax = t;
      }
    }
  }
}
}  // namespace kdtree
