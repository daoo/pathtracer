#ifndef RESTARTTRAVERSER_HPP_WKMTPG5O
#define RESTARTTRAVERSER_HPP_WKMTPG5O

#include "kdtree/linked.hpp"

namespace kdtree {
  namespace traverse {
    namespace restart {
      bool searchTree(const KdTreeLinked& tree, math::Ray& ray, Intersection& isect);
    }
  }
}

#endif /* end of include guard: RESTARTTRAVERSER_HPP_WKMTPG5O */
