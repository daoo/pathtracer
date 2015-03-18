#include "trace/clock.hpp"
#include "trace/kdtree/util.hpp"
#include "trace/scene.hpp"
#include "trace/wavefront/obj.hpp"

#include <boost/filesystem.hpp>
#include <iostream>

using namespace boost::filesystem;
using namespace std;
using namespace trace;
using namespace trace::kdtree;
using namespace util;

ostream& operator<<(ostream& stream, Axis axis)
{
  constexpr char AXIS[] = { 'X', 'Y', 'Z' };
  stream << AXIS[axis];
  return stream;
}

void helper(
    ostream& out,
    const string& label,
    const KdTreeArray& tree,
    unsigned int index,
    Axis axis,
    unsigned int depth)
{
  for (unsigned int i = 0; i < depth; ++i) {
    out << "  ";
  }

  KdTreeArray::Node node = tree.nodes[index];

  if (is_leaf(node)) {
    if (has_triangles(node)) {
      out << "Leaf: " << tree.leaf_store[get_index(node)].size() << "\n";
    } else {
      out << "Leaf: 0\n";
    }
  } else if (is_split(node)) {
    out << "Split: " << label << ", " << axis << ", " << get_split(node) << "\n";
    helper(out, "left", tree, KdTreeArray::left_child(index), next_axis(axis), depth + 1);
    helper(out, "right", tree, KdTreeArray::right_child(index), next_axis(axis), depth + 1);
  } else {
    assert(false && "Node not leaf or split");
  }
}

void print(ostream& out, const KdTreeArray& tree)
{
  helper(out, "root", tree, 0, X, 0);
}

int main(int argc, char* argv[])
{
  if (argc != 2) {
    cout << "Usage: print-tree model.obj\n";
    return 1;
  }

  path obj_file = argv[1];

  const wavefront::Obj obj = wavefront::load_obj(obj_file);
  const wavefront::Mtl mtl = wavefront::load_mtl(obj_file.parent_path() / obj.mtl_lib);

  vector<Triangle> triangles = triangles_from_obj(obj, materials_from_mtl(mtl));

  Clock clock;
  clock.start();
  kdtree::KdTree kdtree = kdtree_from_triangles(triangles);
  clock.stop();

  cout << "Built in " << clock.length<float, ratio<1>>() << " seconds.\n\n";
  print(cout, kdtree);

  return 0;
}
