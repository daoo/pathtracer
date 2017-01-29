#ifndef GEOMETRY_TRIBOX_H_
#define GEOMETRY_TRIBOX_H_

/********************************************************/
/* AABB-triangle overlap test code                      */
/* by Tomas Akenine-Moller                              */
/*                                                      */
/* History:                                             */
/*   2001-03-05: released the code in its first version */
/*   2001-06-18: changed the order of the tests, faster */
/*                                                      */
/* Acknowledgement: Many thanks to Pierre Terdiman for  */
/* suggestions and discussions on how to optimize code. */
/* Thanks to David Hunt for finding a ">="-bug!         */
/********************************************************/

#include <glm/glm.hpp>

#include "geometry/aabb.h"

namespace geometry {
namespace detail {
inline void find_min_max(float x0, float x1, float x2, float& min, float& max) {
  min = glm::min(x0, glm::min(x1, x2));
  max = glm::max(x0, glm::max(x1, x2));
}

inline void test_axis(float normal,
                      float vert,
                      float maxbox,
                      float& vmin,
                      float& vmax) {
  if (normal > 0.0f) {
    vmin = vert - maxbox;
    vmax = maxbox - vert;
  } else {
    vmin = maxbox - vert;
    vmax = vert - maxbox;
  }
}

inline bool plane_box_overlap(const glm::vec3& normal,
                              const glm::vec3& vert,
                              const glm::vec3& maxbox) {
  glm::vec3 vmin, vmax;
  test_axis(normal.x, vert.x, maxbox.x, vmin.x, vmax.x);
  test_axis(normal.y, vert.y, maxbox.y, vmin.y, vmax.y);
  test_axis(normal.z, vert.z, maxbox.z, vmin.z, vmax.z);

  return glm::dot(normal, vmin) <= 0.0f && glm::dot(normal, vmax) >= 0.0f;
}

/*======================== X-tests ========================*/
#define AXISTEST_X01(a, b, fa, fb)           \
  p0 = a * v0.y - b * v0.z;                  \
  p2 = a * v2.y - b * v2.z;                  \
  if (p0 < p2) {                             \
    min = p0;                                \
    max = p2;                                \
  } else {                                   \
    min = p2;                                \
    max = p0;                                \
  }                                          \
  rad = fa * aabb.half.y + fb * aabb.half.z; \
  if (min > rad || max < -rad) return 0;

#define AXISTEST_X2(a, b, fa, fb)            \
  p0 = a * v0.y - b * v0.z;                  \
  p1 = a * v1.y - b * v1.z;                  \
  if (p0 < p1) {                             \
    min = p0;                                \
    max = p1;                                \
  } else {                                   \
    min = p1;                                \
    max = p0;                                \
  }                                          \
  rad = fa * aabb.half.y + fb * aabb.half.z; \
  if (min > rad || max < -rad) return 0;

/*======================== Y-tests ========================*/
#define AXISTEST_Y02(a, b, fa, fb)           \
  p0 = -a * v0.x + b * v0.z;                 \
  p2 = -a * v2.x + b * v2.z;                 \
  if (p0 < p2) {                             \
    min = p0;                                \
    max = p2;                                \
  } else {                                   \
    min = p2;                                \
    max = p0;                                \
  }                                          \
  rad = fa * aabb.half.x + fb * aabb.half.z; \
  if (min > rad || max < -rad) return 0;

#define AXISTEST_Y1(a, b, fa, fb)            \
  p0 = -a * v0.x + b * v0.z;                 \
  p1 = -a * v1.x + b * v1.z;                 \
  if (p0 < p1) {                             \
    min = p0;                                \
    max = p1;                                \
  } else {                                   \
    min = p1;                                \
    max = p0;                                \
  }                                          \
  rad = fa * aabb.half.x + fb * aabb.half.z; \
  if (min > rad || max < -rad) return 0;

/*======================== Z-tests ========================*/

#define AXISTEST_Z12(a, b, fa, fb)           \
  p1 = a * v1.x - b * v1.y;                  \
  p2 = a * v2.x - b * v2.y;                  \
  if (p2 < p1) {                             \
    min = p2;                                \
    max = p1;                                \
  } else {                                   \
    min = p1;                                \
    max = p2;                                \
  }                                          \
  rad = fa * aabb.half.x + fb * aabb.half.y; \
  if (min > rad || max < -rad) return 0;

#define AXISTEST_Z0(a, b, fa, fb)            \
  p0 = a * v0.x - b * v0.y;                  \
  p1 = a * v1.x - b * v1.y;                  \
  if (p0 < p1) {                             \
    min = p0;                                \
    max = p1;                                \
  } else {                                   \
    min = p1;                                \
    max = p0;                                \
  }                                          \
  rad = fa * aabb.half.x + fb * aabb.half.y; \
  if (min > rad || max < -rad) return 0;
}  // namespace detail

inline bool tri_box_overlap(const Aabb& aabb,
                            const glm::vec3& triv0,
                            const glm::vec3& triv1,
                            const glm::vec3& triv2) {
  /* use separating axis theorem to test overlap between triangle and box */
  /* need to test for overlap in these directions: */
  /* 1) the {x, y, z}-directions (actually, since we use the AABB of the */
  /*    triangle we do not even need to test these) */
  /* 2) normal of the triangle */
  /* 3) crossproduct(edge from tri, {x, y, z}-directin) this gives 3x3=9 more */
  /*    tests */
  /* move everything so that the boxcenter is in (0,0,0) */
  glm::vec3 v0, v1, v2;
  v0 = triv0 - aabb.center;
  v1 = triv1 - aabb.center;
  v2 = triv2 - aabb.center;

  /* compute triangle edges */
  glm::vec3 e0, e1, e2;
  e0 = v1 - v0;
  e1 = v2 - v1;
  e2 = v0 - v2;

  float min, max, p0, p1, p2, rad;

  /* Bullet 3:  */
  /* test the 9 tests first (this was faster) */
  {
    float fex = glm::abs<float>(e0.x);
    float fey = glm::abs<float>(e0.y);
    float fez = glm::abs<float>(e0.z);
    AXISTEST_X01(e0.z, e0.y, fez, fey);
    AXISTEST_Y02(e0.z, e0.x, fez, fex);
    AXISTEST_Z12(e0.y, e0.x, fey, fex);
  }

  {
    float fex = glm::abs<float>(e1.x);
    float fey = glm::abs<float>(e1.y);
    float fez = glm::abs<float>(e1.z);
    AXISTEST_X01(e1.z, e1.y, fez, fey);
    AXISTEST_Y02(e1.z, e1.x, fez, fex);
    AXISTEST_Z0(e1.y, e1.x, fey, fex);
  }

  {
    float fex = glm::abs<float>(e2.x);
    float fey = glm::abs<float>(e2.y);
    float fez = glm::abs<float>(e2.z);
    AXISTEST_X2(e2.z, e2.y, fez, fey);
    AXISTEST_Y1(e2.z, e2.x, fez, fex);
    AXISTEST_Z12(e2.y, e2.x, fey, fex);
  }

  /* Bullet 1: */
  /*  first test overlap in the {x, y, z}-directions */
  /*  find min, max of the triangle each direction, and test for overlap in */
  /*  that direction -- this is equivalent to testing a minimal AABB around */
  /*  the triangle against the AABB */

  /* test in X-direction */
  detail::find_min_max(v0.x, v1.x, v2.x, min, max);
  if (min > aabb.half.x || max < -aabb.half.x) return false;

  /* test in Y-direction */
  detail::find_min_max(v0.y, v1.y, v2.y, min, max);
  if (min > aabb.half.y || max < -aabb.half.y) return false;

  /* test in Z-direction */
  detail::find_min_max(v0.z, v1.z, v2.z, min, max);
  if (min > aabb.half.z || max < -aabb.half.z) return false;

  /* Bullet 2: */
  /*  test if the box intersects the plane of the triangle */
  /*  compute plane equation of triangle: normal*x+d=0 */
  glm::vec3 normal = glm::cross(e0, e1);

  return detail::plane_box_overlap(normal, v0, aabb.half);
}
}  // namespace geometry

#endif  // GEOMETRY_TRIBOX_H_
