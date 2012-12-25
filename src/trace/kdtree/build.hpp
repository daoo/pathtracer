#ifndef BUILD_HPP_BTVASI2S
#define BUILD_HPP_BTVASI2S

#include "math/aabb.hpp"
#include "trace/kdtree/linked.hpp"
#include "trace/triangle.hpp"

#include <vector>

namespace trace
{
  namespace kdtree
  {
    void buildTreeSAH(KdTreeLinked::Node*, unsigned int, Axis,
        const math::Aabb&, const std::vector<const Triangle*>& triangles);

    void buildTreeNaive(KdTreeLinked::Node*, unsigned int, Axis,
        const math::Aabb&, const std::vector<const Triangle*>& triangles);
  }
}

#endif /* end of include guard: BUILD_HPP_BTVASI2S */
