#include <iostream>

#include "pathtracer/kdtree/dt/array.hpp"
#include "pathtracer/kdtree/dt/compact.hpp"
#include "pathtracer/kdtree/dt/linked.hpp"

using namespace kdtree;
using namespace std;

int main() {
  cout << "KdTreeLinked::Node\t\t"  << sizeof(KdTreeLinked::Node)  << " bytes\n";
  cout << "KdTreeArray::Node\t\t"   << sizeof(KdTreeArray::Node)   << " bytes\n";
  cout << "KdTreeCompact::Node\t\t" << sizeof(KdTreeCompact::Node) << " bytes\n";

  cout << "std::size_t\t\t\t"                << sizeof(std::size_t)                    << " bytes\n";
  cout << "std::vector<KdTreeArray::Node>\t" << sizeof(std::vector<KdTreeArray::Node>) << " bytes\n";
  return 0;
}
