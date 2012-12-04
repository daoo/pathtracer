#include "linked.hpp"

#include "math/aabb.hpp"

#include <array>
#include <cassert>

using namespace kdtree::helpers;
using namespace math;
using namespace std;

namespace kdtree {
  namespace {
    array<Axis, 3> next = {{ Y, Z, X }};

    void bspBuilder(KdNodeLinked* node, const vector<const Triangle*>& triangles, Axis axis, size_t depth) {
      assert(node != nullptr);
      assert(!triangles.empty());

      if (depth >= 5 || triangles.size() <= 3) {
        node->type = Leaf;

        node->leaf.triangles = new vector<const Triangle*>();
        for (const Triangle* tri : triangles) {
          assert(tri != nullptr);
          node->leaf.triangles->push_back(tri);
        }
      } else {
        Aabb bounding = findBounding(triangles);
        float d = middle(swizzle(bounding.min, axis), swizzle(bounding.max, axis));

        node->type = Split;

        node->split.axis     = axis;
        node->split.distance = d;

        node->split.left  = new KdNodeLinked;
        node->split.right = new KdNodeLinked;

        vector<const Triangle*> left, right;
        for (const Triangle* tri : triangles) {
          if (containsLeft(tri, d, axis)) {
            assert(tri != nullptr);
            left.push_back(tri);
          }

          if (containsRight(tri, d, axis)) {
            assert(tri != nullptr);
            right.push_back(tri);
          }
        }

        assert(left.size() + right.size() >= triangles.size() && "geometry has disappeared");

        bspBuilder(node->split.left, left, next[axis], depth + 1);
        bspBuilder(node->split.right, right, next[axis], depth + 1);
      }
    }

    void printHelper(ostream& out, const KdNodeLinked* node, size_t depth) {
      constexpr char AXIS[] = { 'X', 'Y', 'Z' };

      for (size_t i = 0; i < depth; ++i) {
        out << "  ";
      }

      if (node->type == Leaf) {
        out << "Leaf: " << node->leaf.triangles->size() << "\n";
      } else if (node->type == Split) {
        out << "Split: " << AXIS[node->split.axis] << ", " << node->split.distance << "\n";
        printHelper(out, node->split.left, depth + 1);
        printHelper(out, node->split.right, depth + 1);
      }
    }
  }

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

    vector<const Triangle*> tris;
    for (const Triangle& tri : triangles) {
      tris.push_back(&tri);
    }

    bspBuilder(tree.root, tris, Y, 1);

    cout << "done.\n";
  }

  void print(ostream& out, const KdTreeLinked& tree) {
    printHelper(out, tree.root, 0);
  }
}
