#include <iostream>

#include "pathtracer/kdtree/dt/array.hpp"
#include "pathtracer/kdtree/dt/compact.hpp"
#include "pathtracer/kdtree/dt/linked.hpp"

using namespace kdtree;
using namespace std;

int main() {
  cout << "sizeof(KdTreeLinked::Node) = " << sizeof(KdTreeLinked::Node) << " bytes\n";
  cout << "sizeof(KdTreeArray::Node) = " << sizeof(KdTreeArray::Node) << " bytes\n";
  cout << "sizeof(KdTreeCompact::Node) = " << sizeof(KdTreeCompact::Node) << " bytes\n";
  return 0;
}
