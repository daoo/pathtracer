#include "aabb.hpp"

using namespace std;

namespace math
{
  ostream& operator<<(ostream& stream, const Aabb& aabb)
  {
    stream << "(" << aabb.center.x << ", " << aabb.center.y << ", "
           << aabb.center.z << ")" << " " << "(" << aabb.half.x << ", "
           << aabb.half.y << ", " << aabb.half.z << ")";

    return stream;
  }
}
