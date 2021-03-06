#include <experimental/filesystem>
#include <iostream>

#include "geometry/aap.h"
#include "kdtree/build.h"
#include "kdtree/kdtree.h"
#include "trace/scene.h"
#include "util/clock.h"
#include "util/nicetime.h"
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
            const kdtree::KdNode* node,
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

void print(const kdtree::KdTree& tree) {
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
  kdtree::KdTree kdtree = kdtree::build(triangles);
  double construction_time = clock.measure<double, std::ratio<1>>();
  std::cerr << "Built in " << util::TimeAutoUnit(construction_time) << ".\n";
  print(kdtree);

  return 0;
}
