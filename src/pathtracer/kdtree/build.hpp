#ifndef BUILD_HPP_BTVASI2S
#define BUILD_HPP_BTVASI2S

#include "math/aabb.hpp"
#include "pathtracer/kdtree/linked.hpp"
#include "pathtracer/triangle.hpp"

#include <vector>

namespace kdtree {
  void buildTreeSAH(KdTreeLinked::BuildIter iter, size_t, Axis,
      const math::Aabb& bounding,
      const std::vector<const Triangle*>& triangles);

  void buildTreeNaive(KdTreeLinked::BuildIter iter, size_t, Axis,
      const math::Aabb& box,
      const std::vector<const Triangle*>& triangles);
}

#endif /* end of include guard: BUILD_HPP_BTVASI2S */
