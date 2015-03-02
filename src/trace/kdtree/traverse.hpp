#ifndef TRAVERSE_HPP_IJFE6LBZ
#define TRAVERSE_HPP_IJFE6LBZ

#include "math/ray.hpp"
#include "trace/intersection.hpp"
#include "trace/kdtree/array.hpp"

namespace trace
{
  namespace kdtree
  {
    bool searchTree(
        const KdTreeArray& tree,
        const math::Ray& initray,
        Intersection& isect);
  }
}

#endif /* end of include guard: TRAVERSE_HPP_IJFE6LBZ */
