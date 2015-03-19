#include "trace/camera.hpp"

using namespace glm;

namespace trace
{
  Camera new_camera(
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