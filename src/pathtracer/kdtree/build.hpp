#ifndef BUILD_HPP_BTVASI2S
#define BUILD_HPP_BTVASI2S

#include "math/aabb.hpp"
#include "pathtracer/kdtree/linked.hpp"
#include "pathtracer/triangle.hpp"

#include <vector>

namespace kdtree
{
  math::Aabb findBounding(const std::vector<Triangle>&);

  void buildTreeSAH(KdTreeLinked::Node*, size_t, Axis,
      const math::Aabb&, const std::vector<const Triangle*>& triangles);

  void buildTreeNaive(KdTreeLinked::Node*, size_t, Axis,
      const math::Aabb&, const std::vector<const Triangle*>& triangles);
}

#endif /* end of include guard: BUILD_HPP_BTVASI2S */
