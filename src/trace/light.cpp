#include "trace/light.hpp"

using namespace glm;

namespace trace
{
  SphereLight newLight(
      const vec3& center,
      const vec3& color,
      float intensity,
      float radius)
  {
    return {
      radius,
      center,
      intensity * color
    };
  }
}
