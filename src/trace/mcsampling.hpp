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
    float x = rand.next() * 2.0f - 1.0f;
    float y = rand.next() * 2.0f - 1.0f;

    // Handle degeneracy at the origin
    if (x == 0.0 && y == 0.0) {
      return glm::zero<glm::vec2>();
    }

    float r, theta;
    if (x >= -y) {
      if (x > y) {
        // Handle first region of disk
        r     = x;
        theta = y/x;
      } else {
        // Handle second region of disk
        r     = y;
        theta = 2.0f - x/y;
      }
    } else {
      if (x <= y) {
        // Handle third region of disk
        r     = -x;
        theta = 4.0f + y/x;
      } else {
        // Handle fourth region of disk
        r     = -y;
        theta = 6.0f - x/y;
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
      , glm::sqrt(glm::max(0.0f, 1.0f - ret.x*ret.x - ret.y*ret.y))
      };
  }
}

#endif /* end of include guard: MCSAMPLING_HPP_AF3UKHEV */
