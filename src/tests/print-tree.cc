#include <experimental/filesystem>
#include <iostream>

#include "kdtree/array.h"
#include "kdtree/util.h"
#include "trace/scene.h"
#include "util/clock.h"
#include "wavefront/obj.h"

using std::experimental::filesystem::path;

std::ostream& operator<<(std::ostream& stream, kdtree::Axis axis) {
  constexpr char AXIS[] = {'X', 'Y', 'Z'};
  stream << AXIS[axis];
  return stream;
}

void helper(const std::string& label,
            const kdtree::KdTreeArray& tree,
            unsigned int index,
            kdtree::Axis axis,
            unsigned int depth) {
  for (unsigned int i = 0; i < depth; ++i) {
    std::cout << "  ";
  }

  kdtree::ArrayNode node = tree.get_node(index);

  if (node.is_leaf()) {
    std::cout << "Leaf: " << tree.get_triangles(node).size() << "\n";
  } else if (node.is_split()) {
    std::cout << "Split: " << label << ", " << axis << ", " << node.get_split()
              << "\n";
    helper("left", tree, kdtree::KdTreeArray::left_child(index),
           next_axis(axis), depth + 1);
    helper("right", tree, kdtree::KdTreeArray::right_child(index),
           next_axis(axis), depth + 1);
  } else {
    assert(false && "Node not leaf or split");
  }
}

void print(const kdtree::KdTreeArray& tree) {
  helper("root", tree, 0, kdtree::X, 0);
}

int main(int argc, char* argv[]) {
  if (argc != 2) {
    std::cout << "Usage: print-tree model.obj\n";
    return 1;
  }
  path obj_file = argv[1];

  std::vector<geometry::Triangle> triangles =
      trace::triangles_from_obj(wavefront::load_obj(obj_file));

  util::Clock clock;
  kdtree::KdTreeArray kdtree = trace::kdtree_from_triangles(triangles);
  float construction_time = clock.measure<float, std::ratio<1>>();

  std::cout << "Built in " << construction_time << " seconds.\n\n";
  print(kdtree);

  return 0;
}
