#include "print.hpp"

#include <cassert>
#include <ostream>

using namespace std;

namespace kdtree {
  namespace {
    void helper(ostream& out, const string& label, const KdTreeArray& tree, size_t index, Axis axis, size_t depth) {
      constexpr char AXIS[] = { 'X', 'Y', 'Z' };

      for (size_t i = 0; i < depth; ++i) {
        out << "  ";
      }

      KdTreeArray::Node node = tree.m_nodes[index];

      if (node.isLeaf()) {
        if (node.hasTriangles()) {
          out << "Leaf: " << tree.m_leaf_store[node.getIndex()].size() << "\n";
        } else {
          out << "Leaf: 0\n";
        }
      } else if (node.isSplit()) {
        out << "Split: " << label << ", " << AXIS[axis] << ", " << node.getDistance() << "\n";
        helper(out, "left", tree, KdTreeArray::leftChild(index), nextAxis(axis), depth + 1);
        helper(out, "right", tree, KdTreeArray::rightChild(index), nextAxis(axis), depth + 1);
      } else {
        assert(false && "Node not leaf or split");
      }
    }
  }

  void print(ostream& out, const KdTreeArray& tree) {
    helper(out, "root", tree, 0, X, 0);
  }
}
