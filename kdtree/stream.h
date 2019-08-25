#ifndef KDTREE_STREAM_H_
#define KDTREE_STREAM_H_

#include <iostream>

#include "kdtree/sah_cost.h"

std::ostream& operator<<(std::ostream& stream, const kdtree::Side& value) {
  const char* lookup[] = {"LEFT", "RIGHT"};
  stream << lookup[value];
  return stream;
}

std::ostream& operator<<(std::ostream& stream, const kdtree::Cost& value) {
  stream << "Cost{" << value.cost << "," << value.side << "}";
  return stream;
}

#endif  // KDTREE_STREAM_H_
