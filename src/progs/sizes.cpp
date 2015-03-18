#include <iostream>

#include "trace/kdtree/array.hpp"
#include "trace/kdtree/linked.hpp"

using namespace trace;
using namespace std;

#define PRINT_SIZE(type) cout << #type << " = " << sizeof(type) << " bytes\n"

int main()
{
  PRINT_SIZE(size_t);
  PRINT_SIZE(unsigned int);
  PRINT_SIZE(float);

  PRINT_SIZE(kdtree::KdTreeLinked::Node);
  PRINT_SIZE(kdtree::KdTreeArray::Node);
  PRINT_SIZE(vector<kdtree::KdTreeArray::Node>);
  return 0;
}
