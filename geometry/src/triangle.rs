use glam::{Vec2, Vec3};

use crate::{
    aabb::Aabb, aap::Aap, axial_triangle::AxiallyAlignedTriangle, axis::Axis,
    clip::clip_triangle_aabb, ray::Ray,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

impl Triangle {
    #[inline]
    pub fn base0(&self) -> Vec3 {
        self.v1 - self.v0
    }

    #[inline]
    pub fn base1(&self) -> Vec3 {
        self.v2 - self.v0
    }

    #[inline]
    pub fn base_center(&self) -> Vec3 {
        self.v0 + 0.5 * self.base0() + 0.5 * self.base1()
    }

    #[inline]
    pub fn edge0(&self) -> Vec3 {
        self.v1 - self.v0
    }

    #[inline]
    pub fn edge1(&self) -> Vec3 {
        self.v2 - self.v1
    }

    #[inline]
    pub fn edge2(&self) -> Vec3 {
        self.v0 - self.v2
    }

    #[inline]
    pub fn as_arrays(&self) -> [[f32; 3]; 3] {
        [self.v0.into(), self.v1.into(), self.v2.into()]
    }

    #[inline]
    pub fn min(&self) -> Vec3 {
        self.v0.min(self.v1.min(self.v2))
    }

    #[inline]
    pub fn max(&self) -> Vec3 {
        self.v0.max(self.v1.max(self.v2))
    }

    pub fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb> {
        clip_triangle_aabb(&self.v0, &self.v1, &self.v2, aabb)
    }

    #[inline]
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
    pub fn intersect_ray(&self, ray: &Ray) -> Option<TriangleIntersection> {
        let base1 = self.base0();
        let base2 = self.base1();
        let ray_cross_base2 = ray.direction.cross(base2);

        let det = base1.dot(ray_cross_base2);
        if det == 0.0 {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - self.v0;
        let u = inv_det * s.dot(ray_cross_base2);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let s_cross_base1 = s.cross(base1);
        let v = inv_det * ray.direction.dot(s_cross_base1);
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let t = inv_det * base2.dot(s_cross_base1);
        Some(TriangleIntersection { t, u, v })
    }

    /// Check for overlap using the Separating Axis Theorem.
    pub fn overlaps_aabb(&self, aabb: &Aabb) -> bool {
        let center = aabb.center();
        let half_size = aabb.half_size();

        let v0 = self.v0 - center;
        let v1 = self.v1 - center;
        let v2 = self.v2 - center;

        let f0 = v1 - v0;
        let f1 = v2 - v1;
        let f2 = v0 - v2;

        let test_axis = |axis: Vec3| {
            let p0 = v0.dot(axis);
            let p1 = v1.dot(axis);
            let p2 = v2.dot(axis);
            let r = half_size.x * Vec3::X.dot(axis).abs()
                + half_size.y * Vec3::Y.dot(axis).abs()
                + half_size.z * Vec3::Z.dot(axis).abs();
            (-p0.max(p1.max(p2))).max(p0.min(p1.min(p2))) > r
        };

        if test_axis(Vec3::X.cross(f0)) {
            return false;
        }
        if test_axis(Vec3::X.cross(f1)) {
            return false;
        }
        if test_axis(Vec3::X.cross(f2)) {
            return false;
        }
        if test_axis(Vec3::Y.cross(f0)) {
            return false;
        }
        if test_axis(Vec3::Y.cross(f1)) {
            return false;
        }
        if test_axis(Vec3::Y.cross(f2)) {
            return false;
        }
        if test_axis(Vec3::Z.cross(f0)) {
            return false;
        }
        if test_axis(Vec3::Z.cross(f1)) {
            return false;
        }
        if test_axis(Vec3::Z.cross(f2)) {
            return false;
        }

        if test_axis(Vec3::X) {
            return false;
        }
        if test_axis(Vec3::Y) {
            return false;
        }
        if test_axis(Vec3::Z) {
            return false;
        }

        let triangle_normal = f0.cross(f1);
        if test_axis(triangle_normal) {
            return false;
        }

        true
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriangleIntersection {
    pub t: f32,
    pub u: f32,
    pub v: f32,
}

impl TriangleIntersection {
    pub fn new(t: f32, u: f32, v: f32) -> Self {
        Self { t, u, v }
    }
}

impl<T> From<[T; 3]> for Triangle
where
    T: Into<Vec3> + Copy,
{
    #[inline]
    fn from(value: [T; 3]) -> Self {
        Triangle {
            v0: value[0].into(),
            v1: value[1].into(),
            v2: value[2].into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TriangleNormals {
    pub n0: Vec3,
    pub n1: Vec3,
    pub n2: Vec3,
}

impl TriangleNormals {
    #[inline]
    pub fn lerp(&self, u: f32, v: f32) -> Vec3 {
        ((1.0 - (u + v)) * self.n0 + u * self.n1 + v * self.n2).normalize()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TriangleTexcoords {
    pub uv0: Vec2,
    pub uv1: Vec2,
    pub uv2: Vec2,
}

impl TriangleTexcoords {
    #[inline]
    pub fn lerp(&self, u: f32, v: f32) -> Vec2 {
        (1.0 - (u + v)) * self.uv0 + u * self.uv1 + v * self.uv2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max() {
        let triangle = Triangle {
            v0: Vec3::new(1., 2., 3.),
            v1: Vec3::new(4., 5., 6.),
            v2: Vec3::new(7., 8., 9.),
        };
        assert_eq!(triangle.min(), Vec3::new(1., 2., 3.));
        assert_eq!(triangle.max(), Vec3::new(7., 8., 9.));
    }

    #[test]
    fn center() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 1., 1.),
            v2: Vec3::new(-1., -1., -1.),
        };
        assert_eq!(triangle.base_center(), Vec3::new(0., 0., 0.));
    }

    #[test]
    fn intersect_ray_through_base_center() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 0., 0.),
            v2: Vec3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            Vec3::new(triangle.base_center().x, triangle.base_center().y, -1.),
            Vec3::new(triangle.base_center().x, triangle.base_center().y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(TriangleIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.5,
            })
        );
    }

    #[test]
    fn intersect_ray_through_v0() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 0., 0.),
            v2: Vec3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            Vec3::new(triangle.v0.x, triangle.v0.y, -1.),
            Vec3::new(triangle.v0.x, triangle.v0.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(TriangleIntersection {
                t: 0.5,
                u: 0.,
                v: 0.,
            })
        );
    }

    #[test]
    fn intersect_ray_through_v1() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 0., 0.),
            v2: Vec3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            Vec3::new(triangle.v1.x, triangle.v1.y, -1.),
            Vec3::new(triangle.v1.x, triangle.v1.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(TriangleIntersection {
                t: 0.5,
                u: 1.,
                v: 0.,
            })
        );
    }

    #[test]
    fn intersect_ray_through_v2() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 0., 0.),
            v2: Vec3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            Vec3::new(triangle.v2.x, triangle.v2.y, -1.),
            Vec3::new(triangle.v2.x, triangle.v2.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(TriangleIntersection {
                t: 0.5,
                u: 0.,
                v: 1.,
            })
        );
    }

    #[test]
    fn intersect_ray_through_edge0() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 0., 0.),
            v2: Vec3::new(0., 1., 0.),
        };
        let intersection_point = triangle.v0 + triangle.edge0() / 2.;
        let ray = Ray::between(
            Vec3::new(intersection_point.x, intersection_point.y, -1.),
            Vec3::new(intersection_point.x, intersection_point.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(TriangleIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.,
            })
        );
    }

    #[test]
    fn intersect_ray_through_edge1() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 0., 0.),
            v2: Vec3::new(0., 1., 0.),
        };
        let intersection_point = triangle.v1 + triangle.edge1() / 2.;
        let ray = Ray::between(
            Vec3::new(intersection_point.x, intersection_point.y, -1.),
            Vec3::new(intersection_point.x, intersection_point.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(TriangleIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.5,
            })
        );
    }

    #[test]
    fn intersect_ray_through_edge2() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 0., 0.),
            v2: Vec3::new(0., 1., 0.),
        };
        let intersection_point = triangle.v2 + triangle.edge2() / 2.;
        let ray = Ray::between(
            Vec3::new(intersection_point.x, intersection_point.y, -1.),
            Vec3::new(intersection_point.x, intersection_point.y, 1.),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(TriangleIntersection {
                t: 0.5,
                u: 0.,
                v: 0.5,
            })
        );
    }

    #[test]
    fn intersect_ray_parallel_touching() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 0., 0.),
            v2: Vec3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            Vec3::new(triangle.v0.x, triangle.v0.y, 0.),
            Vec3::new(triangle.v1.x, triangle.v1.y, 0.),
        );

        assert_eq!(triangle.intersect_ray(&ray), None);
    }

    #[test]
    fn intersect_ray_parallel_not_touching() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 0., 0.),
            v2: Vec3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            Vec3::new(triangle.v0.x, triangle.v0.y, 1.),
            Vec3::new(triangle.v1.x, triangle.v1.y, 1.),
        );

        assert_eq!(triangle.intersect_ray(&ray), None);
    }

    #[test]
    fn intersect_ray_almost_parallel_touching() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 0., 0.),
            v2: Vec3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            Vec3::new(triangle.v0.x, triangle.v0.y, -0.000001),
            Vec3::new(triangle.v1.x, triangle.v1.y, 0.000001),
        );

        assert_eq!(
            triangle.intersect_ray(&ray),
            Some(TriangleIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.,
            })
        );
    }

    #[test]
    fn intersect_ray_positive_vs_negative_orientation() {
        let positive = Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(0.0, 1.0, 0.0),
        };
        let negative = Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(0.0, 1.0, 0.0),
            v2: Vec3::new(1.0, 0.0, 0.0),
        };
        let ray = Ray::between(Vec3::new(0.5, 0.0, -1.0), Vec3::new(0.5, 0.0, 1.0));

        assert_eq!(
            positive.intersect_ray(&ray),
            Some(TriangleIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.0,
            })
        );
        assert_eq!(
            negative.intersect_ray(&ray),
            Some(TriangleIntersection {
                t: 0.5,
                u: 0.0,
                v: 0.5,
            })
        );
    }

    #[test]
    fn overlaps_aabb_triangle_completely_inside() {
        let triangle = Triangle {
            v0: Vec3::new(1., 1., 1.),
            v1: Vec3::new(2., 1., 1.),
            v2: Vec3::new(1., 2., 1.),
        };
        let aabb = Aabb::from_extents(Vec3::new(0., 0., 0.), Vec3::new(2., 2., 2.));

        assert!(triangle.overlaps_aabb(&aabb));
    }

    #[test]
    fn overlaps_aabb_triangle_contained_in_one_face() {
        let triangle = Triangle {
            v0: Vec3::new(1., 1., 2.),
            v1: Vec3::new(2., 1., 2.),
            v2: Vec3::new(1., 2., 2.),
        };
        let aabb = Aabb::from_extents(Vec3::new(0., 0., 0.), Vec3::new(2., 2., 2.));

        assert!(triangle.overlaps_aabb(&aabb));
    }

    #[test]
    fn overlaps_aabb_triangle_outside() {
        let triangle = Triangle {
            v0: Vec3::new(10., 10., 10.),
            v1: Vec3::new(11., 10., 10.),
            v2: Vec3::new(10., 11., 10.),
        };
        let aabb = Aabb::from_extents(Vec3::new(0., 0., 0.), Vec3::new(2., 2., 2.));

        assert!(!triangle.overlaps_aabb(&aabb));
    }
}
