use nalgebra::{Vector2, Vector3};

use crate::{
    aabb::Aabb,
    aap::Aap,
    clip::clip_triangle_aabb,
    intersection::{Intersection, RayIntersection},
    ray::Ray,
    Geometry,
};

#[derive(Clone, Debug, PartialEq)]
pub struct AxiallyAlignedTriangle {
    pub plane: Aap,
    pub v0: Vector2<f32>,
    pub v1: Vector2<f32>,
    pub v2: Vector2<f32>,
}

impl AxiallyAlignedTriangle {
    pub fn base0(&self) -> Vector2<f32> {
        self.v1 - self.v0
    }

    pub fn base1(&self) -> Vector2<f32> {
        self.v2 - self.v0
    }

    pub fn param(&self, u: f32, v: f32) -> Vector2<f32> {
        debug_assert!(u >= 0.0 && v >= 0.0 && u + v <= 1.0);
        self.v0 + u * self.base0() + v * self.base1()
    }

    pub fn as_arrays(&self) -> [[f32; 3]; 3] {
        [
            self.plane.add_to(self.v0).into(),
            self.plane.add_to(self.v1).into(),
            self.plane.add_to(self.v2).into(),
        ]
    }

    pub fn intersect_point(&self, point: Vector2<f32>) -> Option<Intersection> {
        let base1 = self.v1 - self.v0;
        let base2 = self.v2 - self.v0;
        let s = point - self.v0;

        let det = base1.x * base2.y - base2.x * base1.y;
        if det == 0.0 {
            return None;
        }

        let inv_det = 1.0 / det;
        let u = inv_det * (s.x * base2.y - base2.x * s.y);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let v = inv_det * (base1.x * s.y - s.x * base1.y);
        if v < 0.0 || v + u > 1.0 {
            return None;
        }

        Some(Intersection { u, v })
    }
}

impl Geometry for AxiallyAlignedTriangle {
    fn min(&self) -> Vector3<f32> {
        let p = self.v0.inf(&self.v1.inf(&self.v2));
        self.plane.axis.add_to(p, self.plane.distance)
    }

    fn max(&self) -> Vector3<f32> {
        let p = self.v0.sup(&self.v1.sup(&self.v2));
        self.plane.axis.add_to(p, self.plane.distance)
    }

    fn intersect_ray(&self, ray: &Ray) -> Option<RayIntersection> {
        let axis = self.plane.axis;
        if ray.direction[axis] == 0.0 {
            return None;
        }
        let t = (self.plane.distance - ray.origin[axis]) / ray.direction[axis];
        let point = ray.param(t);
        self.intersect_point(axis.remove_from(point))
            .map(|intersection| RayIntersection {
                t,
                u: intersection.u,
                v: intersection.v,
            })
    }

    fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb> {
        let clipped = clip_triangle_aabb(
            &self.plane.add_to(self.v0),
            &self.plane.add_to(self.v1),
            &self.plane.add_to(self.v2),
            aabb,
        );

        if clipped.is_empty() {
            return None;
        }
        let start = (clipped[0], clipped[0]);
        let (min, max) = clipped[1..]
            .iter()
            .fold(start, |(min, max), b| (min.inf(b), max.sup(b)));

        Some(Aabb::from_extents(min, max))
    }
}

#[cfg(test)]
mod tests {
    use crate::axis::Axis;

    use super::*;

    const TEST_TRIANGLE: AxiallyAlignedTriangle = AxiallyAlignedTriangle {
        plane: Aap {
            axis: Axis::X,
            distance: 0.0,
        },
        v0: Vector2::new(0.0, 0.0),
        v1: Vector2::new(1.0, 0.0),
        v2: Vector2::new(0.0, 1.0),
    };

    #[test]
    fn intersect_point_outside_triangle() {
        assert_eq!(TEST_TRIANGLE.intersect_point(Vector2::new(2.0, 2.0)), None);
    }

    #[test]
    fn intersect_point_at_v1() {
        assert_eq!(
            TEST_TRIANGLE.intersect_point(Vector2::new(1.0, 0.0)),
            Some(Intersection::new(1.0, 0.0))
        );
    }

    #[test]
    fn intersect_point_at_v2() {
        assert_eq!(
            TEST_TRIANGLE.intersect_point(Vector2::new(0.0, 1.0)),
            Some(Intersection::new(0.0, 1.0))
        );
    }

    #[test]
    fn intersect_point_at_middle_of_edge3() {
        assert_eq!(
            TEST_TRIANGLE.intersect_point(Vector2::new(0.5, 0.5)),
            Some(Intersection::new(0.5, 0.5))
        );
    }

    #[test]
    fn intersect_point_positive_vs_negative_triangle_orientation() {
        let positive = AxiallyAlignedTriangle {
            plane: Aap {
                axis: Axis::X,
                distance: 0.0,
            },
            v0: Vector2::new(0.0, 0.0),
            v1: Vector2::new(1.0, 0.0),
            v2: Vector2::new(0.0, 1.0),
        };
        let negative = AxiallyAlignedTriangle {
            plane: Aap {
                axis: Axis::X,
                distance: 0.0,
            },
            v0: Vector2::new(0.0, 0.0),
            v1: Vector2::new(0.0, 1.0),
            v2: Vector2::new(1.0, 0.0),
        };
        let point = Vector2::new(0.5, 0.0);

        assert_eq!(
            positive.intersect_point(point),
            Some(Intersection::new(0.5, 0.0))
        );
        assert_eq!(
            negative.intersect_point(point),
            Some(Intersection::new(0.0, 0.5))
        );
    }
}
