#include "trace/camera.hpp"

using namespace glm;

namespace trace
{
  Camera newCamera(
      const vec3& position,
      const vec3& target,
      const vec3& approxup,
      float fov)
  {
    return {
      position,
      normalize(target - position),
      normalize(approxup),
      fov
    };
  }
}
