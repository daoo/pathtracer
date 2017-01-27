#include <iostream>

#include "kdtree/array.hpp"
#include "kdtree/linked.hpp"

using namespace trace;
using namespace std;

#define PRINT_SIZE(type) cout << #type << " = " << sizeof(type) << " bytes\n"

int main() {
  PRINT_SIZE(size_t);
  PRINT_SIZE(unsigned int);
  PRINT_SIZE(float);

  PRINT_SIZE(kdtree::LinkedNode);
  PRINT_SIZE(kdtree::ArrayNode);
  PRINT_SIZE(vector<kdtree::ArrayNode>);
  return 0;
}
