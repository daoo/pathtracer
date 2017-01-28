#ifndef TRAVERSE_HPP_IJFE6LBZ
#define TRAVERSE_HPP_IJFE6LBZ

namespace geometry {
struct Ray;
}

namespace kdtree {
class KdTreeArray;
struct Intersection;
bool search_tree(const KdTreeArray& tree,
                 const geometry::Ray& ray,
                 float tmin,
                 float tmax,
                 Intersection& isect);
}

#endif /* end of include guard: TRAVERSE_HPP_IJFE6LBZ */
