#include "kdtree/array.h"
#include "kdtree/util.h"
#include "trace/scene.h"
#include "util/clock.h"
#include "wavefront/obj.h"

#include <experimental/filesystem>
#include <iostream>

using namespace kdtree;
using namespace std::experimental::filesystem;
using namespace std;

ostream& operator<<(ostream& stream, Axis axis) {
  constexpr char AXIS[] = {'X', 'Y', 'Z'};
  stream << AXIS[axis];
  return stream;
}

void helper(const string& label,
            const KdTreeArray& tree,
            unsigned int index,
            Axis axis,
            unsigned int depth) {
  for (unsigned int i = 0; i < depth; ++i) {
    cout << "  ";
  }

  ArrayNode node = tree.get_node(index);

  if (node.is_leaf()) {
    cout << "Leaf: " << tree.get_triangles(node).size() << "\n";
  } else if (node.is_split()) {
    cout << "Split: " << label << ", " << axis << ", " << node.get_split()
         << "\n";
    helper("left", tree, KdTreeArray::left_child(index), next_axis(axis),
           depth + 1);
    helper("right", tree, KdTreeArray::right_child(index), next_axis(axis),
           depth + 1);
  } else {
    assert(false && "Node not leaf or split");
  }
}

void print(const KdTreeArray& tree) {
  helper("root", tree, 0, X, 0);
}

int main(int argc, char* argv[]) {
  if (argc != 2) {
    cout << "Usage: print-tree model.obj\n";
    return 1;
  }
  path obj_file = argv[1];

  vector<geometry::Triangle> triangles =
    trace::triangles_from_obj(wavefront::load_obj(obj_file));

  util::Clock clock;
  kdtree::KdTreeArray kdtree = trace::kdtree_from_triangles(triangles);
  float construction_time = clock.measure<float, ratio<1>>();

  cout << "Built in " << construction_time << " seconds.\n\n";
  print(kdtree);

  return 0;
}
