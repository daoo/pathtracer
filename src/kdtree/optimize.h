#ifndef KDTREE_OPTIMIZE_H_
#define KDTREE_OPTIMIZE_H_

#include "kdtree/linked.h"

namespace kdtree {
class KdTreeArray;
void optimize(KdTreeArray&, const KdTreeLinked&);
}  // namespace kdtree

#endif  // KDTREE_OPTIMIZE_H_
