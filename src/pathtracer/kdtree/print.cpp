#include "print.hpp"

#include <cassert>
#include <ostream>

using namespace std;

namespace kdtree
{
  namespace
  {
    ostream& operator<<(ostream& stream, Axis axis)
    {
      constexpr char AXIS[] = { 'X', 'Y', 'Z' };
      stream << AXIS[axis];
      return stream;
    }

    void helper(ostream& out, const string& label, const KdTreeArray& tree,
        unsigned int index, Axis axis, unsigned int depth)
    {
      for (unsigned int i = 0; i < depth; ++i) {
        out << "  ";
      }

      KdTreeArray::Node node = tree.nodes[index];

      if (isLeaf(node)) {
        if (hasTriangles(node)) {
          out << "Leaf: " << tree.leaf_store[getIndex(node)].size() << "\n";
        } else {
          out << "Leaf: 0\n";
        }
      } else if (isSplit(node)) {
        out << "Split: " << label << ", " << axis << ", " << getSplit(node) << "\n";
        helper(out, "left", tree, KdTreeArray::leftChild(index), nextAxis(axis), depth + 1);
        helper(out, "right", tree, KdTreeArray::rightChild(index), nextAxis(axis), depth + 1);
      } else {
        assert(false && "Node not leaf or split");
      }
    }
  }

  void print(ostream& out, const KdTreeArray& tree)
  {
    helper(out, "root", tree, 0, X, 0);
  }
}
