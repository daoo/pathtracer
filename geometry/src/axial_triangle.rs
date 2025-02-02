use glam::{Vec2, Vec3};

use crate::{
    aabb::Aabb,
    aap::Aap,
    clip::clip_triangle_aabb,
    intersection::{PointIntersection, RayIntersection},
    ray::Ray,
};

#[derive(Clone, Debug, PartialEq)]
pub struct AxiallyAlignedTriangle {
    pub plane: Aap,
    pub v0: Vec2,
    pub v1: Vec2,
    pub v2: Vec2,
}

impl AxiallyAlignedTriangle {
    #[inline]
    pub fn min(&self) -> Vec3 {
        let p = self.v0.min(self.v1.min(self.v2));
        self.plane.axis.add_to(p, self.plane.distance)
    }

    #[inline]
    pub fn max(&self) -> Vec3 {
        let p = self.v0.max(self.v1.max(self.v2));
        self.plane.axis.add_to(p, self.plane.distance)
    }

    #[inline]
    pub fn base0(&self) -> Vec2 {
        self.v1 - self.v0
    }

    #[inline]
    pub fn base1(&self) -> Vec2 {
        self.v2 - self.v0
    }

    #[inline]
    pub fn param(&self, u: f32, v: f32) -> Vec2 {
        debug_assert!(u >= 0.0 && v >= 0.0 && u + v <= 1.0);
        self.v0 + u * self.base0() + v * self.base1()
    }

    #[inline]
    pub fn as_arrays(&self) -> [[f32; 3]; 3] {
        [
            self.plane.add_to(self.v0).into(),
            self.plane.add_to(self.v1).into(),
            self.plane.add_to(self.v2).into(),
        ]
    }

    pub fn intersect_point(&self, point: Vec2) -> Option<PointIntersection> {
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

        Some(PointIntersection::new(u, v))
    }

    pub fn intersect_ray(&self, ray: &Ray) -> Option<RayIntersection> {
        let axis = self.plane.axis;
        if ray.direction[axis] == 0.0 {
            return None;
        }
        let t = (self.plane.distance - ray.origin[axis]) / ray.direction[axis];
        let point = ray.param(t);
        self.intersect_point(axis.remove_from(point))
            .map(|intersection| intersection.with_ray_param(t))
    }

    pub fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb> {
        clip_triangle_aabb(
            &self.plane.add_to(self.v0),
            &self.plane.add_to(self.v1),
            &self.plane.add_to(self.v2),
            aabb,
        )
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
        v0: Vec2::new(0.0, 0.0),
        v1: Vec2::new(1.0, 0.0),
        v2: Vec2::new(0.0, 1.0),
    };

    #[test]
    fn intersect_point_outside_triangle() {
        assert_eq!(TEST_TRIANGLE.intersect_point(Vec2::new(2.0, 2.0)), None);
    }

    #[test]
    fn intersect_point_at_v1() {
        assert_eq!(
            TEST_TRIANGLE.intersect_point(Vec2::new(1.0, 0.0)),
            Some(PointIntersection::new(1.0, 0.0))
        );
    }

    #[test]
    fn intersect_point_at_v2() {
        assert_eq!(
            TEST_TRIANGLE.intersect_point(Vec2::new(0.0, 1.0)),
            Some(PointIntersection::new(0.0, 1.0))
        );
    }

    #[test]
    fn intersect_point_at_middle_of_edge3() {
        assert_eq!(
            TEST_TRIANGLE.intersect_point(Vec2::new(0.5, 0.5)),
            Some(PointIntersection::new(0.5, 0.5))
        );
    }

    #[test]
    fn intersect_point_positive_vs_negative_triangle_orientation() {
        let positive = AxiallyAlignedTriangle {
            plane: Aap {
                axis: Axis::X,
                distance: 0.0,
            },
            v0: Vec2::new(0.0, 0.0),
            v1: Vec2::new(1.0, 0.0),
            v2: Vec2::new(0.0, 1.0),
        };
        let negative = AxiallyAlignedTriangle {
            plane: Aap {
                axis: Axis::X,
                distance: 0.0,
            },
            v0: Vec2::new(0.0, 0.0),
            v1: Vec2::new(0.0, 1.0),
            v2: Vec2::new(1.0, 0.0),
        };
        let point = Vec2::new(0.5, 0.0);

        assert_eq!(
            positive.intersect_point(point),
            Some(PointIntersection::new(0.5, 0.0))
        );
        assert_eq!(
            negative.intersect_point(point),
            Some(PointIntersection::new(0.0, 0.5))
        );
    }
}
