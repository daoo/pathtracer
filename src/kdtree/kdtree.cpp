#include "kdtree.hpp"

#include "math/aabb.hpp"

#include <array>

using namespace math;
using namespace std;

namespace {
  array<Axis, 3> next = {{ Y, Z, X }};

  void bspBuilder(KdNode* node, const vector<const Triangle*>& triangles, Axis axis, size_t depth) {
    if (depth >= 20 || triangles.size() <= 3) {
      node->type = Leaf;
      node->leaf.triangles = triangles;
      return;
    }

    Aabb bounding = findBounding(triangles);
    float d = middle(swizzle(bounding.min, axis), swizzle(bounding.max, axis));

    node->type         = Parent;
    node->parent.left  = new KdNode;
    node->parent.right = new KdNode;

    vector<const Triangle*> left, right;
    for (const Triangle* tri : triangles) {
      if (containsLeft(tri, d, axis)) {
        left.push_back(tri);
      }

      if (containsRight(tri, d, axis)) {
        right.push_back(tri);
      }
    }

    assert(left.size() + right.size() >= triangles.size());

    bspBuilder(node->parent.left, left, next[axis], depth + 1);
    bspBuilder(node->parent.right, right, next[axis], depth + 1);
  }

  bool intersectionFinder(const KdNode* node, Ray& ray, Intersection& isect) {
    assert(node != nullptr);

    switch (node->type) {
      case Leaf: {
        bool foundIntersection = false;
        for (size_t i = 0; i < node->leaf.triangles.size(); ++i) {
          foundIntersection |= findIntersection(*node->leaf.triangles[i], ray, isect);
        }
        return foundIntersection;
      } case Parent: {
        return intersectionFinder(node->parent.left, ray, isect)
            || intersectionFinder(node->parent.right, ray, isect);
      }
    }
  }
}

KdTree buildTree(const Scene& scene) {
  KdTree tree;

  tree.root = new KdNode;

  vector<const Triangle*> triangles;
  for (const Triangle& tri : scene.m_triangles) {
    triangles.push_back(&tri);
  }

  bspBuilder(tree.root, triangles, X, 1);

  return tree;
}

bool intersectsTree(const KdTree& tree, Ray& ray, Intersection& isect) {
  return intersectionFinder(tree.root, ray, isect);
}
