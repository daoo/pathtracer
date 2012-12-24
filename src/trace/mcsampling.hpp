#ifndef MCSAMPLING_HPP_AF3UKHEV
#define MCSAMPLING_HPP_AF3UKHEV

#include "trace/fastrand.hpp"
#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

namespace trace
{
  inline glm::vec3 uniformSampleSphere(FastRand& rand, float radius) {
    float z = rand() * 2.0f - 1.0f;
    float a = rand() * 6.283185307179586476925286766559f;

    float r = glm::sqrt(1.0f - z * z);

    float x = r * glm::cos(a);
    float y = r * glm::sin(a);

    return glm::vec3(x, y, z) * radius;
  }

  inline glm::vec3 uniformSampleHemisphere(FastRand& rand)
  {
    float r1 = rand();
    float r2 = rand();

    float a = sqrtf(r2 * (1.0f - r2));
    return glm::vec3(
        2.0f * glm::cos(glm::pi<float>() * 2.0f * r1) * a,
        2.0f * glm::sin(glm::pi<float>() * 2.0f * r1) * a,
        std::fabs(1.0f - 2.0f * r2));
  }

  inline void concentricSampleDisk(FastRand& rand, float& dx, float& dy)
  {
    float sx = rand() * 2.0f - 1.0f;
    float sy = rand() * 2.0f - 1.0f;
    // Map uniform random numbers to $[-1,1]^2$
    // Map square to $(r,\theta)$
    // Handle degeneracy at the origin
    if (sx == 0.0 && sy == 0.0) {
      dx = 0.0;
      dy = 0.0;
      return;
    }

    float r, theta;
    if (sx >= -sy) {
      if (sx > sy) {
        // Handle first region of disk
        r = sx;
        if (sy > 0.0) theta = sy/r;
        else          theta = 8.0f + sy/r;
      } else {
        // Handle second region of disk
        r = sy;
        theta = 2.0f - sx/r;
      }
    } else {
      if (sx <= sy) {
        // Handle third region of disk
        r = -sx;
        theta = 4.0f - sy/r;
      }
      else {
        // Handle fourth region of disk
        r = -sy;
        theta = 6.0f + sx/r;
      }
    }
    theta *= glm::pi<float>() / 4.f;
    dx = r * glm::cos(theta);
    dy = r * glm::sin(theta);
  }

  inline glm::vec3 cosineSampleHemisphere(FastRand& engine)
  {
    glm::vec3 ret;
    concentricSampleDisk(engine, ret.x, ret.y);
    ret.z = sqrtf(std::max(0.f, 1.f - ret.x*ret.x - ret.y*ret.y));
    return ret;
  }
}

#endif /* end of include guard: MCSAMPLING_HPP_AF3UKHEV */
