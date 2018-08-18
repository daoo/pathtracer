#include <experimental/filesystem>
#include <iostream>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/bounding.h"
#include "kdtree/build.h"
#include "kdtree/linked.h"
#include "trace/scene.h"
#include "wavefront/obj.h"

#include "geometry/stream.h"

using geometry::Aabb;
using geometry::Aap;
using geometry::Triangle;
using glm::vec3;
using std::vector;
using std::experimental::filesystem::path;

void PrintSplit(std::ostream& stream,
                int depth,
                const Aap& plane,
                const vec3& min,
                const vec3& max) {
  float x1, y1, x2, y2;
  if (plane.GetAxis() == geometry::X) {
    x1 = plane.GetDistance();
    x2 = plane.GetDistance();
    y1 = min.y;
    y2 = max.y;
  } else if (plane.GetAxis() == geometry::Y) {
    x1 = min.x;
    x2 = max.x;
    y1 = plane.GetDistance();
    y2 = plane.GetDistance();
  } else {
    return;
  }
  stream << "<line ";
  stream << " x1=\"" << x1 << "\" y1=\"" << y1 << "\"";
  stream << " x2=\"" << x2 << "\" y2=\"" << y2 << "\"";
  glm::vec3 colors[]{
      {255, 0, 0},
      {0, 255, 0},
      {0, 0, 255},
  };
  glm::vec3 c = colors[depth % 3];
  stream << " style=\"stroke:rgb(" << c.r << "," << c.g << "," << c.b
         << ");stroke-width:1\" />";
}

void PrintTriangle(std::ostream& stream, const geometry::Triangle& triangle) {
  stream << "<polygon points=\"";
  stream << triangle.v0.x << "," << triangle.v0.y << " " << triangle.v1.x;
  stream << ",";
  stream << triangle.v1.y << " " << triangle.v2.x << "," << triangle.v2.y;
  stream << "\" />";
}

void helper(int depth,
            const kdtree::KdNodeLinked* node,
            const vec3& min,
            const vec3& max) {
  if (node->GetTriangles() != nullptr) {
    for (auto& triangle : *node->GetTriangles()) {
      PrintTriangle(std::cout, *triangle);
    }
  } else {
    PrintSplit(std::cout, depth, node->GetPlane(), min, max);
    {
      vec3 newmax = max;
      newmax[node->GetPlane().GetAxis()] = node->GetPlane().GetDistance();
      helper(depth + 1, node->GetLeft(), min, newmax);
    }
    {
      vec3 newmin = min;
      newmin[node->GetPlane().GetAxis()] = node->GetPlane().GetDistance();
      helper(depth + 1, node->GetRight(), newmin, max);
    }
  }
}

void print(const kdtree::KdTreeLinked& tree, const Aabb& bounding) {
  std::cout << "<svg>";
  helper(0, tree.GetRoot(), bounding.GetMin(), bounding.GetMax());
  std::cout << "</svg>\n";
}

int main(int argc, char* argv[]) {
  if (argc != 2) {
    std::cerr << "Usage: print-tree model.obj\n";
    return 1;
  }
  path obj_file = argv[1];

  vector<Triangle> triangles =
      trace::triangles_from_obj(wavefront::LoadObj(obj_file));

  Aabb bounding = geometry::find_bounding(triangles);
  kdtree::KdTreeLinked kdtree = kdtree::build(triangles);
  print(kdtree, bounding);

  return 0;
}
