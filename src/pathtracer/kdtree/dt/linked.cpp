#include "linked.hpp"

#include "kdtree/build/halfsplits.hpp"
#include "math/aabb.hpp"

#include <array>
#include <cassert>

using namespace kdtree::helpers;
using namespace math;
using namespace std;

namespace kdtree {
  KdNodeLinked::KdNodeLinked() { }
  KdNodeLinked::~KdNodeLinked() {
    if (m_type == Leaf) {
      delete m_leaf.m_triangles;
    } else if (m_type == Split) {
      delete m_split.m_left;
      delete m_split.m_right;
    }
  }

  KdTreeLinked::KdTreeLinked() { }
  KdTreeLinked::~KdTreeLinked() { delete root; }

  void buildKdTreeLinked(KdTreeLinked& tree, const vector<Triangle>& triangles) {
    assert(!triangles.empty());

    cout << "Building Kd-tree... ";

    tree.root = new KdNodeLinked;

    vector<Triangle> tris;
    for (const Triangle& tri : triangles) {
      tris.push_back(tri);
    }

    build::halfSplitsBuildTree(KdTreeLinked::BuildIter(tree), tris);

    cout << "done.\n";
  }
}
