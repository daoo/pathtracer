#include <iostream>

#include "pathtracer/kdtree/array.hpp"
#include "pathtracer/kdtree/linked.hpp"

using namespace kdtree;
using namespace std;

int main() {
  cout << "KdTreeLinked::Node\t\t"  << sizeof(KdTreeLinked::Node)  << " bytes\n";
  cout << "KdTreeArray::Node\t\t"   << sizeof(KdTreeArray::Node)   << " bytes\n";

  cout << "std::size_t\t\t\t"                << sizeof(std::size_t)                    << " bytes\n";
  cout << "std::vector<KdTreeArray::Node>\t" << sizeof(std::vector<KdTreeArray::Node>) << " bytes\n";
  return 0;
}
