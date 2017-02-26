#ifndef KDTREE_OPTIMIZE_H_
#define KDTREE_OPTIMIZE_H_

namespace kdtree {
class KdTreeLinked;
class KdTreeArray;
KdTreeArray optimize(const KdTreeLinked&);
}  // namespace kdtree

#endif  // KDTREE_OPTIMIZE_H_
