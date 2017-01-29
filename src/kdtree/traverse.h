#ifndef KDTREE_TRAVERSE_H_
#define KDTREE_TRAVERSE_H_

namespace geometry {
struct Ray;
}  // namespace geometry

namespace kdtree {
class KdTreeArray;
struct Intersection;
bool search_tree(const KdTreeArray& tree,
                 const geometry::Ray& ray,
                 float tmin,
                 float tmax,
                 Intersection& isect);
}  // namespace kdtree

#endif  // KDTREE_TRAVERSE_H_
