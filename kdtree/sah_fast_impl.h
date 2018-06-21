#ifndef KDTREE_SAH_FAST_IMPL_H_
#define KDTREE_SAH_FAST_IMPL_H_

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

#include <cassert>
#include <set>
#include <vector>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "geometry/split.h"
#include "kdtree/build_common.h"
#include "kdtree/linked.h"
#include "kdtree/sah_common.h"

namespace {

enum Side { LEFT, RIGHT };

struct KdCost {
  float cost;
  Side side;

  bool operator<(const KdCost& other) const { return cost < other.cost; }
};

struct KdCostSplit {
  geometry::Aap plane;
  KdCost cost;
};

KdCost CalculateCost(const geometry::Aabb& parent,
                     const geometry::Aap& plane,
                     size_t left_count,
                     size_t right_count,
                     size_t plane_count) {
  float parent_area = parent.GetSurfaceArea();
  geometry::AabbSplit split = geometry::Split(parent, plane);
  float left_area = split.left.GetSurfaceArea();
  float right_area = split.right.GetSurfaceArea();
  float plane_left =
      kdtree::CalculateCost(parent_area, left_area, right_area,
                            left_count + plane_count, right_count);
  float plane_right =
      kdtree::CalculateCost(parent_area, left_area, right_area, left_count,
                            right_count + plane_count);
  return plane_left < plane_right ? KdCost{plane_left, LEFT}
                                  : KdCost{plane_right, RIGHT};
}

enum Type { START, PLANAR, END };

struct Event {
  geometry::Aap plane;
  Type type;

  bool operator<(const Event& other) const {
    return plane < other.plane || (plane == other.plane && type < other.type);
  }
};

void ListPerfectSplits(const geometry::Aabb& boundary,
                       const geometry::Triangle& triangle,
                       geometry::Axis axis,
                       std::set<Event>* splits) {
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

void ListPerfectSplits(const geometry::Aabb& boundary,
                       const geometry::Triangle& triangle,
                       std::set<Event>* splits) {
  ListPerfectSplits(boundary, triangle, geometry::X, splits);
  ListPerfectSplits(boundary, triangle, geometry::Y, splits);
  ListPerfectSplits(boundary, triangle, geometry::Z, splits);
}

std::set<Event> ListPerfectSplits(const kdtree::KdBox& box) {
  std::set<Event> splits;
  for (const geometry::Triangle* triangle : box.triangles) {
    ListPerfectSplits(box.boundary, *triangle, &splits);
  }
  return splits;
}

struct EventCount {
  size_t pminus, pplus, pplane;
};

EventCount CountEvents(const std::set<Event>& splits,
                       const Event& event,
                       std::set<Event>::const_iterator iter) {
  size_t pminus = 0;
  size_t pplus = 0;
  size_t pplane = 0;
  while (iter != splits.end() &&
         iter->plane.GetDistance() == event.plane.GetDistance() &&
         event.type == START) {
    pminus += 1;
    ++iter;
  }
  while (iter != splits.end() &&
         iter->plane.GetDistance() == event.plane.GetDistance() &&
         event.type == PLANAR) {
    pplane += 1;
    ++iter;
  }
  while (iter != splits.end() &&
         iter->plane.GetDistance() == event.plane.GetDistance() &&
         event.type == END) {
    pplus += 1;
    ++iter;
  }
  return EventCount{pminus, pplus, pplane};
}

KdCostSplit FindBestSplit(const kdtree::KdBox& parent,
                          const std::set<Event>& splits) {
  assert(splits.size() > 0);
  KdCostSplit best{{geometry::X, 0}, {FLT_MAX, LEFT}};
  for (int axis_index = 0; axis_index < 3; ++axis_index) {
    geometry::Axis axis = static_cast<geometry::Axis>(axis_index);
    size_t nl = 0;
    size_t nr = parent.triangles.size();
    for (std::set<Event>::const_iterator iter = splits.begin();
         iter != splits.end(); ++iter) {
      if (iter->plane.GetAxis() == axis) {
        EventCount count = CountEvents(splits, *iter, iter);
        nr = nr - count.pminus - count.pplane;
        KdCost cost =
            CalculateCost(parent.boundary, iter->plane, nl, nr, count.pplane);
        if (cost < best.cost) {
          best = KdCostSplit{iter->plane, cost};
        }
        nl = nl + count.pplus + count.pplane;
      }
    }
  }
  return best;
}

kdtree::KdNodeLinked* BuildHelper(unsigned int depth,
                                  const kdtree::KdBox& parent) {
  // sizeof(kdtree::KdNodeLinked) * node count = 32 * 2^20 = 32 MB
  if (depth >= 20 || parent.triangles.empty()) {
    return new kdtree::KdNodeLinked(
        new std::vector<const geometry::Triangle*>(parent.triangles));
  }

  std::set<Event> splits = ListPerfectSplits(parent);
  if (splits.empty()) {
    return new kdtree::KdNodeLinked(
        new std::vector<const geometry::Triangle*>(parent.triangles));
  }

  KdCostSplit split = FindBestSplit(parent, splits);
  if (split.cost.cost > kdtree::LeafCostBound(parent.triangles.size())) {
    return new kdtree::KdNodeLinked(
        new std::vector<const geometry::Triangle*>(parent.triangles));
  } else {
    kdtree::KdSplit boxes = Split(parent, split.plane);
    return new kdtree::KdNodeLinked(split.plane,
                                    BuildHelper(depth + 1, boxes.left),
                                    BuildHelper(depth + 1, boxes.right));
  }
}

}  // namespace

#endif  // KDTREE_SAH_FAST_IMPL_H_
