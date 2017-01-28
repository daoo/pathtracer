#ifndef TRAVERSE_HPP_IJFE6LBZ
#define TRAVERSE_HPP_IJFE6LBZ

#include "geometry/ray.hpp"
#include "kdtree/array.hpp"
#include "kdtree/intersection.hpp"

namespace kdtree {
bool search_tree(const KdTreeArray& tree,
                 const geometry::Ray& ray,
                 float tmin,
                 float tmax,
                 Intersection& isect);
}

#endif /* end of include guard: TRAVERSE_HPP_IJFE6LBZ */
