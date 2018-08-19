#include <glm/glm.hpp>

#include <iterator>
#include <set>

#include "geometry/aabb.h"
#include "geometry/aap.h"
#include "geometry/triangle.h"
#include "kdtree/sah_fast_impl.h"
#include "third_party/catch/catch.h"

using kdtree::Event;

template <class T>
T advance(T iter, size_t steps) {
  std::advance(iter, steps);
  return iter;
}

TEST_CASE("CountEvents example", "[SAH]") {
  std::set<Event> events{{{geometry::X, 0}, kdtree::START},
                         {{geometry::X, 1}, kdtree::END},
                         {{geometry::X, 2}, kdtree::START},
                         {{geometry::X, 3}, kdtree::END}};

  REQUIRE(CountEvents(advance(events.cbegin(), 0), events.cend()).pplus == 1);
  REQUIRE(CountEvents(advance(events.cbegin(), 1), events.cend()).pminus == 1);
  REQUIRE(CountEvents(advance(events.cbegin(), 2), events.cend()).pplus == 1);
  REQUIRE(CountEvents(advance(events.cbegin(), 3), events.cend()).pminus == 1);
}
