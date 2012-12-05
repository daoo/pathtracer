#include "linked.hpp"

#include "kdtree/build/boundingmiddle.hpp"
#include "math/aabb.hpp"

#include <array>
#include <cassert>

using namespace kdtree::helpers;
using namespace math;
using namespace std;

namespace kdtree {
  KdNodeLinked::KdNodeLinked() { }
  KdNodeLinked::~KdNodeLinked() {
    if (type == Leaf) {
      delete leaf.triangles;
    } else if (type == Split) {
      delete split.left;
      delete split.right;
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

    build::boundingMiddleBuildTree(KdTreeLinked::BuildIter(tree), tris);

    cout << "done.\n";
  }
}
