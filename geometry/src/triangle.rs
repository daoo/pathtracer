use arrayvec::ArrayVec;
use nalgebra::Vector3;

use crate::{
    aabb::Aabb, aap::Aap, axial_triangle::AxiallyAlignedTriangle, axis::Axis,
    clip::clip_triangle_aabb, intersection::RayIntersection, ray::Ray,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub v0: Vector3<f32>,
    pub v1: Vector3<f32>,
    pub v2: Vector3<f32>,
}

impl Triangle {
    pub fn min(&self) -> Vector3<f32> {
        self.v0.inf(&self.v1.inf(&self.v2))
    }

    pub fn max(&self) -> Vector3<f32> {
        self.v0.sup(&self.v1.sup(&self.v2))
    }

    pub fn base0(&self) -> Vector3<f32> {
        self.v1 - self.v0
    }

    pub fn base1(&self) -> Vector3<f32> {
        self.v2 - self.v0
    }

    pub fn base_center(&self) -> Vector3<f32> {
        self.v0 + 0.5 * self.base0() + 0.5 * self.base1()
    }

    pub fn edge0(&self) -> Vector3<f32> {
        self.v1 - self.v0
    }

    pub fn edge1(&self) -> Vector3<f32> {
        self.v2 - self.v1
    }

    pub fn edge2(&self) -> Vector3<f32> {
        self.v0 - self.v2
    }

    pub fn param(&self, u: f32, v: f32) -> Vector3<f32> {
        assert!(u >= 0.0 && v >= 0.0 && u + v <= 1.0);
        self.v0 + u * self.base0() + v * self.base1()
    }

    pub fn from_arrays(arrays: [[f32; 3]; 3]) -> Triangle {
        let [v0, v1, v2] = arrays;
        Triangle {
            v0: v0.into(),
            v1: v1.into(),
            v2: v2.into(),
        }
    }

    pub fn as_arrays(self) -> [[f32; 3]; 3] {
        [self.v0.into(), self.v1.into(), self.v2.into()]
    }

    pub fn as_axially_aligned(&self) -> Option<AxiallyAlignedTriangle> {
        let check_axis = |axis| {
            (self.v0[axis] == self.v1[axis] && self.v0[axis] == self.v2[axis]).then_some(
                AxiallyAlignedTriangle {
                    plane: Aap {
                        axis,
                        distance: self.v0[axis],
                    },
                    v0: axis.remove_from(self.v0),
                    v1: axis.remove_from(self.v1),
                    v2: axis.remove_from(self.v2),
                },
            )
        };

        check_axis(Axis::X)
            .or(check_axis(Axis::Y))
            .or(check_axis(Axis::Z))
    }

    /// Compute triangle-ray intersection using the Möller–Trumbore algorithm.
    pub fn intersect_ray(&self, ray: &Ray) -> Option<RayIntersection> {
        let base1 = self.base0();
        let base2 = self.base1();
        let ray_cross_base2 = ray.direction.cross(&base2);

        let det = base1.dot(&ray_cross_base2);
        if det == 0.0 {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - self.v0;
        let u = inv_det * s.dot(&ray_cross_base2);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let s_cross_base1 = s.cross(&base1);
        let v = inv_det * ray.direction.dot(&s_cross_base1);
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let t = inv_det * base2.dot(&s_cross_base1);
        Some(RayIntersection { t, u, v })
    }

    /// Check for overlap using the Separating Axis Theorem.
    pub fn overlaps_aabb(&self, aabb: &Aabb) -> bool {
        const U0: Vector3<f32> = Vector3::new(1., 0., 0.);
        const U1: Vector3<f32> = Vector3::new(0., 1., 0.);
        const U2: Vector3<f32> = Vector3::new(0., 0., 1.);

        let center = aabb.center();
        let half_size = aabb.half_size();

        let v0 = self.v0 - center;
        let v1 = self.v1 - center;
        let v2 = self.v2 - center;

        let f0 = v1 - v0;
        let f1 = v2 - v1;
        let f2 = v0 - v2;

        let test_axis = |axis: Vector3<f32>| {
            let p0 = v0.dot(&axis);
            let p1 = v1.dot(&axis);
            let p2 = v2.dot(&axis);
            let r = half_size.x * U0.dot(&axis).abs()
                + half_size.y * U1.dot(&axis).abs()
                + half_size.z * U2.dot(&axis).abs();
            (-p0.max(p1.max(p2))).max(p0.min(p1.min(p2))) > r
        };

        if test_axis(U0.cross(&f0)) {
            return false;
        }
        if test_axis(U0.cross(&f1)) {
            return false;
        }
        if test_axis(U0.cross(&f2)) {
            return false;
        }
        if test_axis(U1.cross(&f0)) {
            return false;
        }
        if test_axis(U1.cross(&f1)) {
            return false;
        }
        if test_axis(U1.cross(&f2)) {
            return false;
        }
        if test_axis(U2.cross(&f0)) {
            return false;
        }
        if test_axis(U2.cross(&f1)) {
            return false;
        }
        if test_axis(U2.cross(&f2)) {
            return false;
        }

        if test_axis(U0) {
            return false;
        }
        if test_axis(U1) {
            return false;
        }
        if test_axis(U2) {
            return false;
        }

        let triangle_normal = f0.cross(&f1);
        if test_axis(triangle_normal) {
            return false;
        }

        true
    }

    pub fn clip_aabb(&self, aabb: &Aabb) -> ArrayVec<Vector3<f32>, 6> {
        clip_triangle_aabb(&self.v0, &self.v1, &self.v2, aabb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max() {
        let triangle = Triangle {
            v0: Vector3::new(1., 2., 3.),
            v1: Vector3::new(4., 5., 6.),
            v2: Vector3::new(7., 8., 9.),
        };
        assert_eq!(triangle.min(), Vector3::new(1., 2., 3.));
        assert_eq!(triangle.max(), Vector3::new(7., 8., 9.));
    }

    #[test]
    fn center() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 1., 1.),
            v2: Vector3::new(-1., -1., -1.),
        };
        assert_eq!(triangle.base_center(), Vector3::new(0., 0., 0.));
    }

    #[test]
    fn intersect_ray_through_base_center() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            &Vector3::new(triangle.base_center().x, triangle.base_center().y, -1.),
            &Vector3::new(triangle.base_center().x, triangle.base_center().y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(RayIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.5
            })
        );
    }

    #[test]
    fn intersect_ray_through_v0() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            &Vector3::new(triangle.v0.x, triangle.v0.y, -1.),
            &Vector3::new(triangle.v0.x, triangle.v0.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(RayIntersection {
                t: 0.5,
                u: 0.,
                v: 0.
            })
        );
    }

    #[test]
    fn intersect_ray_through_v1() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            &Vector3::new(triangle.v1.x, triangle.v1.y, -1.),
            &Vector3::new(triangle.v1.x, triangle.v1.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(RayIntersection {
                t: 0.5,
                u: 1.,
                v: 0.
            })
        );
    }

    #[test]
    fn intersect_ray_through_v2() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            &Vector3::new(triangle.v2.x, triangle.v2.y, -1.),
            &Vector3::new(triangle.v2.x, triangle.v2.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(RayIntersection {
                t: 0.5,
                u: 0.,
                v: 1.
            })
        );
    }

    #[test]
    fn intersect_ray_through_edge0() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let intersection_point = triangle.v0 + triangle.edge0() / 2.;
        let ray = Ray::between(
            &Vector3::new(intersection_point.x, intersection_point.y, -1.),
            &Vector3::new(intersection_point.x, intersection_point.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(RayIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.
            })
        );
    }

    #[test]
    fn intersect_ray_through_edge1() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let intersection_point = triangle.v1 + triangle.edge1() / 2.;
        let ray = Ray::between(
            &Vector3::new(intersection_point.x, intersection_point.y, -1.),
            &Vector3::new(intersection_point.x, intersection_point.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(RayIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.5
            })
        );
    }

    #[test]
    fn intersect_ray_through_edge2() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let intersection_point = triangle.v2 + triangle.edge2() / 2.;
        let ray = Ray::between(
            &Vector3::new(intersection_point.x, intersection_point.y, -1.),
            &Vector3::new(intersection_point.x, intersection_point.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(RayIntersection {
                t: 0.5,
                u: 0.,
                v: 0.5
            })
        );
    }

    #[test]
    fn intersect_ray_parallel_touching() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            &Vector3::new(triangle.v0.x, triangle.v0.y, 0.),
            &Vector3::new(triangle.v1.x, triangle.v1.y, 0.),
        );

        assert_eq!(triangle.intersect_ray(&ray), None);
    }

    #[test]
    fn intersect_ray_parallel_not_touching() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            &Vector3::new(triangle.v0.x, triangle.v0.y, 1.),
            &Vector3::new(triangle.v1.x, triangle.v1.y, 1.),
        );

        assert_eq!(triangle.intersect_ray(&ray), None);
    }

    #[test]
    fn intersect_ray_almost_parallel_touching() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            &Vector3::new(triangle.v0.x, triangle.v0.y, -0.000001),
            &Vector3::new(triangle.v1.x, triangle.v1.y, 0.000001),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(RayIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.
            })
        );
    }

    #[test]
    fn intersect_ray_positive_vs_negative_orientation() {
        let positive = Triangle {
            v0: Vector3::new(0.0, 0.0, 0.0),
            v1: Vector3::new(1.0, 0.0, 0.0),
            v2: Vector3::new(0.0, 1.0, 0.0),
        };
        let negative = Triangle {
            v0: Vector3::new(0.0, 0.0, 0.0),
            v1: Vector3::new(0.0, 1.0, 0.0),
            v2: Vector3::new(1.0, 0.0, 0.0),
        };
        let ray = Ray::between(&Vector3::new(0.5, 0.0, -1.0), &Vector3::new(0.5, 0.0, 1.0));

        assert_eq!(
            positive.intersect_ray(&ray),
            Some(RayIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.0
            })
        );
        assert_eq!(
            negative.intersect_ray(&ray),
            Some(RayIntersection {
                t: 0.5,
                u: 0.0,
                v: 0.5
            })
        );
    }

    #[test]
    fn overlaps_aabb_triangle_completely_inside() {
        let triangle = Triangle {
            v0: Vector3::new(1., 1., 1.),
            v1: Vector3::new(2., 1., 1.),
            v2: Vector3::new(1., 2., 1.),
        };
        let aabb = Aabb::from_extents(Vector3::new(0., 0., 0.), Vector3::new(2., 2., 2.));

        assert_eq!(triangle.overlaps_aabb(&aabb), true);
    }

    #[test]
    fn overlaps_aabb_triangle_contained_in_one_face() {
        let triangle = Triangle {
            v0: Vector3::new(1., 1., 2.),
            v1: Vector3::new(2., 1., 2.),
            v2: Vector3::new(1., 2., 2.),
        };
        let aabb = Aabb::from_extents(Vector3::new(0., 0., 0.), Vector3::new(2., 2., 2.));

        assert_eq!(triangle.overlaps_aabb(&aabb), true);
    }

    #[test]
    fn overlaps_aabb_triangle_outside() {
        let triangle = Triangle {
            v0: Vector3::new(10., 10., 10.),
            v1: Vector3::new(11., 10., 10.),
            v2: Vector3::new(10., 11., 10.),
        };
        let aabb = Aabb::from_extents(Vector3::new(0., 0., 0.), Vector3::new(2., 2., 2.));

        assert_eq!(triangle.overlaps_aabb(&aabb), false);
    }
}
