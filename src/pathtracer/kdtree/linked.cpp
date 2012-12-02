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

      if (depth >= 20 || triangles.size() <= 3) {
        node->type = KdNodeLinked::Leaf;

        node->leaf.triangles = new vector<const Triangle*>(triangles.size());
        for (const Triangle* tri : triangles) {
          assert(tri != nullptr);
          node->leaf.triangles->push_back(tri);
        }

        return;
      }

      Aabb bounding = findBounding(triangles);
      float d = middle(swizzle(bounding.min, axis), swizzle(bounding.max, axis));

      node->type         = KdNodeLinked::Parent;
      node->parent.left  = new KdNodeLinked;
      node->parent.right = new KdNodeLinked;

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

      assert(left.size() + right.size() >= triangles.size());

      bspBuilder(node->parent.left, left, next[axis], depth + 1);
      bspBuilder(node->parent.right, right, next[axis], depth + 1);
    }

    bool intersectsHelper(const KdNodeLinked* node, Ray& ray, Intersection& isect) {
      assert(node != nullptr);

      switch (node->type) {
        case KdNodeLinked::Leaf: {
          bool foundIntersection = false;

          std::vector<const Triangle*> tris = *node->leaf.triangles;
          for (size_t i = 0; i < tris.size(); ++i) {
            foundIntersection |= intersects(*tris[i], ray, isect);
          }
          return foundIntersection;
        } case KdNodeLinked::Parent: {
          return intersectsHelper(node->parent.left, ray, isect)
              || intersectsHelper(node->parent.right, ray, isect);
        }
      }
    }
  }

  KdNodeLinked::KdNodeLinked() { }
  KdNodeLinked::~KdNodeLinked() {
    if (type == Leaf) {
      delete leaf.triangles;
    } else if (type == Parent) {
      delete parent.left;
      delete parent.right;
    }
  }

  KdTreeLinked::KdTreeLinked() { }
  KdTreeLinked::~KdTreeLinked() { delete root; }

  void buildKdTreeLinked(KdTreeLinked& tree, const vector<Triangle>& triangles) {
    assert(!triangles.empty());

    tree.root = new KdNodeLinked;

    vector<const Triangle*> tris;
    for (const Triangle& tri : triangles) {
      tris.push_back(&tri);
    }

    bspBuilder(tree.root, tris, X, 1);
  }

  bool intersects(const KdTreeLinked& tree, Ray& ray, Intersection& isect) {
    return intersectsHelper(tree.root, ray, isect);
  }

  bool intersects(const KdTreeLinked& tree, const Ray& ray) {
    Intersection isect;
    Ray raycopy(ray);
    return intersectsHelper(tree.root, raycopy, isect);
  }
}
