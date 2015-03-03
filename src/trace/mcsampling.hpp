#ifndef MCSAMPLING_HPP_AF3UKHEV
#define MCSAMPLING_HPP_AF3UKHEV

#include "trace/fastrand.hpp"
#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>

namespace trace
{
  inline glm::vec2 uniformSampleSquare(FastRand& rand)
  {
    return glm::vec2(rand.next(), rand.next());
  }

  inline glm::vec3 uniformSampleSphere(FastRand& rand)
  {
    float z = rand.next() * 2.0f - 1.0f;
    float a = rand.next() * (2.0f * glm::pi<float>());

    float r = glm::sqrt(1.0f - z * z);

    float x = r * glm::cos(a);
    float y = r * glm::sin(a);

    return glm::vec3(x, y, z);
  }

  inline glm::vec3 uniformSampleHemisphere(FastRand& rand)
  {
    glm::vec2 r = uniformSampleSquare(rand);

    float a = 2.0f * glm::sqrt(r.y * (1.0f - r.y));
    float b = glm::pi<float>() * 2.0f * r.x;
    return glm::vec3
      { a * glm::cos(b)
      , a * glm::sin(b)
      , glm::abs(1.0f - 2.0f * r.y)
      };
  }

  inline glm::vec2 concentricSampleDisk(FastRand& rand)
  {
    float sx = rand.next() * 2.0f - 1.0f;
    float sy = rand.next() * 2.0f - 1.0f;

    // Handle degeneracy at the origin
    if (sx == 0.0 && sy == 0.0) {
      return glm::zero<glm::vec2>();
    }

    float r, theta;
    if (sx >= -sy) {
      if (sx > sy) {
        // Handle first region of disk
        r     = sx;
        theta = sy > 0.0f ? sy / r : 8.0f + sy / r;
      } else {
        // Handle second region of disk
        r     = sy;
        theta = 2.0f - sx / r;
      }
    } else {
      if (sx <= sy) {
        // Handle third region of disk
        r     = -sx;
        theta = 4.0f - sy / r;
      } else {
        // Handle fourth region of disk
        r     = -sy;
        theta = 6.0f + sx / r;
      }
    }

    theta *= glm::quarter_pi<float>();
    return glm::vec2
      { r * glm::cos(theta)
      , r * glm::sin(theta)
      };
  }

  inline glm::vec3 cosineSampleHemisphere(FastRand& engine)
  {
    glm::vec2 ret = concentricSampleDisk(engine);
    return glm::vec3
      { ret.x
      , ret.y
      , glm::sqrt(
          glm::max(
            0.0f, 1.0f - ret.x * ret.x - ret.y * ret.y))
      };
  }
}

#endif /* end of include guard: MCSAMPLING_HPP_AF3UKHEV */
