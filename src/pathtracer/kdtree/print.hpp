#ifndef PRINT_HPP_HABMENIW
#define PRINT_HPP_HABMENIW

#include <ostream>

namespace kdtree {
  namespace detail {
    template <typename Iter>
    void printHelper(std::ostream& out, Iter iter, size_t depth) {
      constexpr char AXIS[] = { 'X', 'Y', 'Z' };

      for (size_t i = 0; i < depth; ++i) {
        out << "  ";
      }

      if (iter.isLeaf()) {
        out << "Leaf: " << iter.triangles().size() << "\n";
      } else if (iter.isSplit()) {
        out << "Split: " << AXIS[iter.axis()] << ", " << iter.split() << "\n";
        printHelper(out, iter.left(), depth + 1);
        printHelper(out, iter.right(), depth + 1);
      }
    }
  }

  template <typename Tree>
  void print(std::ostream& out, const Tree& tree) {
    detail::printHelper(out, typename Tree::Iterator(tree.root), 0);
  }
}

#endif /* end of include guard: PRINT_HPP_HABMENIW */

