#include <algorithm>
#include <glm/glm.hpp>
#include <iostream>
#include <iterator>
#include <string>

#include "kdtree/build.h"
#include "kdtree/linked.h"
#include "trace/fastrand.h"
#include "util/clock.h"
#include "util/nicetime.h"

using geometry::Triangle;
using kdtree::KdNodeLinked;
using std::vector;
using trace::FastRand;

namespace {

glm::vec3 RandomVec3(FastRand& rand) {
  return glm::vec3(rand.range(-10.0f, 10.0f), rand.range(-10.0f, 10.0f),
                   rand.range(-10.0f, 10.0f));
}

Triangle RandomTriangle(FastRand& rand) {
  Triangle tri;
  tri.v0 = RandomVec3(rand);
  tri.v1 = RandomVec3(rand);
  tri.v2 = RandomVec3(rand);
  return tri;
}

bool Contains(const KdNodeLinked* node, const Triangle* triangle) {
  if (node->GetTriangles() != nullptr) {
    const vector<const Triangle*>* triangles = node->GetTriangles();
    return std::find(std::begin(*triangles), std::end(*triangles), triangle) !=
           std::end(*triangles);
  } else {
    return Contains(node->GetLeft(), triangle) ||
           Contains(node->GetRight(), triangle);
  }
}

}  // namespace

int main(int argc, char** argv) {
  if (argc != 3) {
    std::cerr << "Usage: " << argv[0]
              << " [triangle count] [number of tests]\n";
    return 1;
  }

  size_t triangle_count = std::stoul(argv[1]);
  size_t test_count = std::stoul(argv[2]);

  FastRand rand;

  util::Clock clock;
  for (size_t i = 0; i < test_count; ++i) {
    vector<Triangle> triangles;
    for (size_t j = 0; j < triangle_count; ++j) {
      triangles.emplace_back(RandomTriangle(rand));
    }
    kdtree::KdTreeLinked kdtree = kdtree::build(triangles);
    for (size_t j = 0; j < triangle_count; ++j) {
      if (!Contains(kdtree.GetRoot(), &triangles[j])) {
        std::cout << "Error: triangle not found!\n";
        return 2;
      }
    }
  }

  double test_time = clock.measure<double, std::ratio<1>>();
  std::cout << "Tested " << test_count * triangle_count
            << " triangles successfully in "
            << util::TimeAutoUnit(test_time) << ".\n";
  return 0;
}
