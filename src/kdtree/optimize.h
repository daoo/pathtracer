#ifndef KDTREE_OPTIMIZE_H_
#define KDTREE_OPTIMIZE_H_

namespace kdtree {
struct KdNodeLinked;
class KdTreeArray;
KdTreeArray optimize(const KdNodeLinked*);
}  // namespace kdtree

#endif  // KDTREE_OPTIMIZE_H_
