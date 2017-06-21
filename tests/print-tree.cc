#include <experimental/filesystem>
#include <iostream>

#include "geometry/aap.h"
#include "kdtree/linked.h"
#include "kdtree/surface_area_heuristic.h"
#include "kdtree/util.h"
#include "trace/scene.h"
#include "util/clock.h"
#include "wavefront/obj.h"

using std::experimental::filesystem::path;

std::ostream& operator<<(std::ostream& stream, geometry::Axis axis) {
  constexpr char AXIS[] = {'X', 'Y', 'Z'};
  stream << AXIS[axis];
  return stream;
}

std::ostream& operator<<(std::ostream& stream, geometry::Aap plane) {
  stream << plane.GetAxis() << "@" << plane.GetDistance();
  return stream;
}

void helper(const std::string& label,
            const kdtree::KdNodeLinked* node,
            unsigned int depth) {
  std::cout << std::string(depth * 2, ' ');
  if (node->GetTriangles() != nullptr) {
    std::cout << "Leaf: " << node->GetTriangles()->size() << "\n";
  } else {
    std::cout << "Split: " << label << ", " << node->GetPlane() << "\n";
    helper("left", node->GetLeft(), depth + 1);
    helper("right", node->GetRight(), depth + 1);
  }
}

void print(const kdtree::KdTreeLinked& tree) {
  helper("root", tree.GetRoot(), 0);
}

int main(int argc, char* argv[]) {
  if (argc != 2) {
    std::cout << "Usage: print-tree model.obj\n";
    return 1;
  }
  path obj_file = argv[1];

  std::vector<geometry::Triangle> triangles =
      trace::triangles_from_obj(wavefront::LoadObj(obj_file));

  util::Clock clock;
  kdtree::KdTreeLinked kdtree = kdtree::build_tree_sah(triangles);
  float construction_time = clock.measure<float, std::ratio<1>>();
  std::cerr << "Built in " << construction_time << "s.\n";
  print(kdtree);

  return 0;
}
