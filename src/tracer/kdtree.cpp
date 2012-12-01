#include "kdtree.hpp"

#include "math/aabb.hpp"

#include <array>

using namespace math;
using namespace std;

namespace {
  array<Axis, 3> next = {{ Y, Z, X }};

  float swizzle(const glm::vec3& v, Axis c) {
    switch (c) {
      case X: return v.x;
      case Y: return v.y;
      case Z: return v.z;
    }
  }

  float middle(float a, float b) {
    return a + (b - a) / 2.0f;
  }

  Aabb findBounding(const std::vector<const Triangle*>& triangles) {
    glm::vec3 min, max;

    for (const Triangle* tri : triangles) {
      min = glm::min(min, tri->v0);
      min = glm::min(min, tri->v1);
      min = glm::min(min, tri->v2);

      max = glm::max(max, tri->v0);
      max = glm::max(max, tri->v1);
      max = glm::max(max, tri->v2);
    }

    return { min, max };
  }

  bool containsLeft(const Triangle* tri, float d, Axis axis) {
    return swizzle(tri->v0, axis) < d
        || swizzle(tri->v1, axis) < d
        || swizzle(tri->v2, axis) < d;
  }

  bool containsRight(const Triangle* tri, float d, Axis axis) {
    return swizzle(tri->v0, axis) > d
        || swizzle(tri->v1, axis) > d
        || swizzle(tri->v2, axis) > d;
  }

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

    bspBuilder(node->parent.left, left, next[axis], depth + 1);
    bspBuilder(node->parent.right, right, next[axis], depth + 1);
  }

  bool intersectionFinder(const KdNode* node, Ray& ray, Intersection& isect) {
    if (node->type == Leaf) {
      bool foundIntersection = false;
      for (size_t i = 0; i < node->leaf.triangles.size(); ++i) {
        foundIntersection |= findIntersection(*node->leaf.triangles[i], ray, isect);
      }
      return foundIntersection;
    } else if (node->type == Parent) {
      return false;
    }

    return false;
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
