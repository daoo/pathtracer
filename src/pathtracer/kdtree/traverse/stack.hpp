#ifndef STACKTRAVERSER_HPP_JCTSK9YD
#define STACKTRAVERSER_HPP_JCTSK9YD

#include "kdtree/linked.hpp"

namespace kdtree {
  namespace traverse {
    namespace stack {
      bool searchTree(const KdTreeLinked& tree, math::Ray& ray, Intersection& isect);
    }
  }
}

#endif /* end of include guard: STACKTRAVERSER_HPP_JCTSK9YD */
