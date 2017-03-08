#ifndef GEOMETRY_AAP_H_
#define GEOMETRY_AAP_H_

namespace geometry {
enum Axis { X = 0, Y = 1, Z = 2 };

// Axis-aligned plane.
class Aap {
 public:
  Aap(Axis axis, float distance) : axis_(axis), distance_(distance) {}

  Axis GetAxis() const { return axis_; }
  float GetDistance() const { return distance_; }

 private:
  Axis axis_;
  float distance_;
};
}  // namespace geometry

#endif  // GEOMETRY_AAP_H_
