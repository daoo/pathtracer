#include <iostream>

#include "kdtree/array.h"
#include "kdtree/linked.h"

#define PRINT_SIZE(type) \
  std::cout << #type << " = " << sizeof(type) << " bytes\n"

int main() {
  PRINT_SIZE(size_t);
  PRINT_SIZE(unsigned int);
  PRINT_SIZE(float);

  PRINT_SIZE(kdtree::KdNodeLinked);
  PRINT_SIZE(kdtree::KdNodeArray);
  return 0;
}
