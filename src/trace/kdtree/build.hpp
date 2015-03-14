#ifndef BUILD_HPP_BTVASI2S
#define BUILD_HPP_BTVASI2S

#include "trace/geometry/aabb.hpp"
#include "trace/geometry/triangle.hpp"
#include "trace/kdtree/linked.hpp"

#include <vector>

namespace trace
{
  namespace kdtree
  {
    void build_tree_sah(
        KdTreeLinked::Node*,
        unsigned int,
        Axis,
        const Aabb&,
        const std::vector<const Triangle*>& triangles);

    void build_tree_naive(
        KdTreeLinked::Node*,
        unsigned int,
        Axis,
        const Aabb&,
        const std::vector<const Triangle*>& triangles);
  }
}

#endif /* end of include guard: BUILD_HPP_BTVASI2S */
