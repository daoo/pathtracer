#include "kdtree/build.h"

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include <cassert>
#include <set>
#include <vector>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "geometry/split.h"
#include "kdtree/intersect.h"
#include "kdtree/linked.h"

namespace geometry {
struct Triangle;
}  // namespace geometry

using geometry::Aabb;
using geometry::AabbSplit;
using geometry::Aap;
using geometry::Axis;
using geometry::Triangle;
using glm::vec3;
using kdtree::KdNodeLinked;
using std::set;
using std::vector;

namespace {

constexpr float COST_TRAVERSE = 0.01f;
constexpr float COST_INTERSECT = 1.0f;
constexpr float COST_EMPTY = 0.8f;

float calculate_cost(float parent_area,
                     float left_area,
                     float right_area,
                     size_t left_count,
                     size_t right_count) {
  float factor = left_count == 0 || right_count == 0 ? COST_EMPTY : 1.0f;
  float intersect =
      (left_area * left_count + right_area * right_count) / parent_area;
  return factor * COST_TRAVERSE + COST_INTERSECT * intersect;
}

enum Side { LEFT, RIGHT };

struct Cost {
  float cost;
  Side side;

  bool operator<(const Cost& other) const { return cost < other.cost; }
};

Cost calculate_cost(const Aabb& parent,
                    const Aap& plane,
                    size_t left_count,
                    size_t right_count,
                    size_t plane_count) {
  float parent_area = parent.GetSurfaceArea();
  AabbSplit split = geometry::split(parent, plane);
  float left_area = split.left.GetSurfaceArea();
  float right_area = split.right.GetSurfaceArea();
  float plane_left = calculate_cost(parent_area, left_area, right_area,
                                    left_count + plane_count, right_count);
  float plane_right = calculate_cost(parent_area, left_area, right_area,
                                     left_count, right_count + plane_count);
  return plane_left < plane_right ? Cost{plane_left, LEFT}
                                  : Cost{plane_right, RIGHT};
}

enum Type { START, PLANAR, END };

struct Event {
  Aap plane;
  Type type;

  bool operator<(const Event& other) const {
    return plane < other.plane || (plane == other.plane && type < other.type);
  }
};

void list_perfect_splits(const Aabb& boundary,
                         const Triangle& triangle,
                         Axis axis,
                         set<Event>* splits) {
  float boundary_min = boundary.GetMin()[axis];
  float boundary_max = boundary.GetMax()[axis];
  float triangle_min = triangle.GetMin()[axis];
  float triangle_max = triangle.GetMax()[axis];
  float clamped_min = glm::clamp(triangle_min, boundary_min, boundary_max);
  float clamped_max = glm::clamp(triangle_max, boundary_min, boundary_max);
  if (clamped_min == clamped_max) {
    splits->insert({{axis, clamped_min}, PLANAR});
  } else {
    splits->insert({{axis, clamped_min}, START});
    splits->insert({{axis, clamped_max}, END});
  }
}

void list_perfect_splits(const Aabb& boundary,
                         const Triangle& triangle,
                         set<Event>* splits) {
  list_perfect_splits(boundary, triangle, geometry::X, splits);
  list_perfect_splits(boundary, triangle, geometry::Y, splits);
  list_perfect_splits(boundary, triangle, geometry::Z, splits);
}

struct Box {
  Aabb boundary;
  std::vector<const Triangle*> triangles;
};

set<Event> list_perfect_splits(const Box& box) {
  set<Event> splits;
  for (const Triangle* triangle : box.triangles) {
    list_perfect_splits(box.boundary, *triangle, &splits);
  }
  return splits;
}

struct CostSplit {
  Aap plane;
  Cost cost;
};

CostSplit find_best_split(const Box& parent, const set<Event>& splits) {
  assert(splits.size() > 0);
  CostSplit best{{geometry::X, 0}, {FLT_MAX, LEFT}};
  for (int axis_index = 0; axis_index < 3; ++axis_index) {
    Axis axis = static_cast<Axis>(axis_index);
    int nl = 0;
    int np = 0;
    int nr = parent.triangles.size();
    for (set<Event>::const_iterator outer = splits.begin();
         outer != splits.end(); ++outer) {
      const Event& event = *outer;
      const Aap& plane = event.plane;
      if (plane.GetAxis() == axis) {
        set<Event>::const_iterator inner = outer;
        int pminus = 0;
        int pplus = 0;
        int pplane = 0;
        while (inner != splits.end() &&
               inner->plane.GetDistance() == plane.GetDistance() &&
               event.type == START) {
          pminus += 1;
          ++inner;
        }
        while (inner != splits.end() &&
               inner->plane.GetDistance() == plane.GetDistance() &&
               event.type == PLANAR) {
          pplane += 1;
          ++inner;
        }
        while (inner != splits.end() &&
               inner->plane.GetDistance() == plane.GetDistance() &&
               event.type == END) {
          pplus += 1;
          ++inner;
        }
        np = pplane;
        nr = nr - pplane - pminus;
        Cost cost = calculate_cost(parent.boundary, plane, nl, nr, np);
        if (cost < best.cost) {
          best = CostSplit{plane, cost};
        }
        nl = nl + pplus + pplane;
        np = 0;
      }
    }
  }
  return best;
}

struct Split {
  Box left, right;
};

Split split_triangles(const Box& parent, const Aap& plane) {
  AabbSplit aabbs = geometry::split(parent.boundary, plane);
  kdtree::IntersectResults triangles =
      kdtree::intersect_test(parent.triangles, plane);
  Box left{aabbs.left, triangles.left};
  Box right{aabbs.right, triangles.right};
  return {left, right};
}

KdNodeLinked* go(unsigned int depth, const Box& parent) {
  // sizeof(KdNodeLinked) * node count = 32 * 2^20 = 32 MB
  if (depth >= 20 || parent.triangles.empty()) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  }

  set<Event> splits = list_perfect_splits(parent);
  if (splits.empty()) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  }

  CostSplit split = find_best_split(parent, splits);
  float leaf_cost = COST_INTERSECT * parent.triangles.size();
  if (split.cost.cost > leaf_cost) {
    return new KdNodeLinked(new vector<const Triangle*>(parent.triangles));
  } else {
    Split boxes = split_triangles(parent, split.plane);
    return new KdNodeLinked(split.plane, go(depth + 1, boxes.left),
                            go(depth + 1, boxes.right));
  }
}
}  // namespace

namespace kdtree {
KdTreeLinked build(const vector<Triangle>& triangles) {
  vector<const Triangle*> ptrs;
  ptrs.reserve(triangles.size());
  for (const Triangle& triangle : triangles) {
    ptrs.emplace_back(&triangle);
  }

  return KdTreeLinked(go(0, Box{find_bounding(triangles), ptrs}));
}
}  // namespace kdtree
