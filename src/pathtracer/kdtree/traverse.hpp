#ifndef TRAVERSE_HPP_IJFE6LBZ
#define TRAVERSE_HPP_IJFE6LBZ

#include "math/ray.hpp"
#include "pathtracer/intersection.hpp"
#include "pathtracer/kdtree/array.hpp"

namespace kdtree {
  bool searchTree(const KdTreeArray& tree, math::Ray& ray, Intersection& isect);
}

#endif /* end of include guard: TRAVERSE_HPP_IJFE6LBZ */
