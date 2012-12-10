#ifndef PRINT_HPP_HABMENIW
#define PRINT_HPP_HABMENIW

#include <cassert>
#include <ostream>

namespace kdtree {
  namespace detail {
    template <typename Iter>
    void printHelper(std::ostream& out, const std::string& label, const Iter& iter, size_t depth) {
      constexpr char AXIS[] = { 'X', 'Y', 'Z' };

      for (size_t i = 0; i < depth; ++i) {
        out << "  ";
      }

      if (iter.isLeaf()) {
        if (iter.hasTriangles()) {
          out << "Leaf: " << iter.triangles().size() << "\n";
        } else {
          out << "Leaf: 0\n";
        }
      } else if (iter.isSplit()) {
        out << "Split: " << label << ", " << AXIS[iter.axis()] << ", " << iter.split() << "\n";
        printHelper(out, "left", iter.left(), depth + 1);
        printHelper(out, "right", iter.right(), depth + 1);
      } else {
        assert(false && "Node not leaf or split");
      }
    }
  }

  template <typename Tree>
  void print(std::ostream& out, const Tree& tree) {
    detail::printHelper(out, "root", typename Tree::TraverseIter(tree), 0);
  }
}

#endif /* end of include guard: PRINT_HPP_HABMENIW */
