#ifndef TRAVERSER_HPP_NIQVU4JM
#define TRAVERSER_HPP_NIQVU4JM

#include "kdtree/linked.hpp"

namespace kdtree {
  namespace traverse {
    namespace stack {
      bool searchTree(const KdTreeLinked& tree, math::Ray& ray, Intersection& isect);
    }
  }
}

#endif /* end of include guard: TRAVERSER_HPP_NIQVU4JM */
