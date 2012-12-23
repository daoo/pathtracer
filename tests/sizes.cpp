#include <iostream>

#include "pathtracer/kdtree/array.hpp"
#include "pathtracer/kdtree/linked.hpp"

using namespace kdtree;
using namespace std;

#define PRINT_SIZE(type) cout << #type << " = " << sizeof(type) << " bytes\n"

int main()
{
  PRINT_SIZE(size_t);
  PRINT_SIZE(unsigned int);
  PRINT_SIZE(float);

  PRINT_SIZE(KdTreeLinked::Node);
  PRINT_SIZE(KdTreeArray::Node);
  PRINT_SIZE(vector<KdTreeArray::Node>);
  return 0;
}
