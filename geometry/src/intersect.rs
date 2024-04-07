use nalgebra::Vector3;

use super::{aabb::Aabb, triangle::Triangle};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub u: f32,
    pub v: f32,
}

impl Intersection {
    pub fn new(u: f32, v: f32) -> Intersection {
        Intersection { u, v }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayIntersection {
    pub t: f32,
    pub u: f32,
    pub v: f32,
}

impl RayIntersection {
    pub fn new(t: f32, u: f32, v: f32) -> Self {
        RayIntersection { t, u, v }
    }
}

pub fn intersect_triangle_aabb(triangle: &Triangle, aabb: &Aabb) -> bool {
    const U0: Vector3<f32> = Vector3::new(1., 0., 0.);
    const U1: Vector3<f32> = Vector3::new(0., 1., 0.);
    const U2: Vector3<f32> = Vector3::new(0., 0., 1.);

    let center = aabb.center();
    let half_size = aabb.half_size();

    let v0 = triangle.v0 - center;
    let v1 = triangle.v1 - center;
    let v2 = triangle.v2 - center;

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

#[cfg(test)]
mod tests_intersect_triangle_aabb {
    use super::*;

    #[test]
    fn triangle_completely_inside() {
        let triangle = Triangle {
            v0: Vector3::new(1., 1., 1.),
            v1: Vector3::new(2., 1., 1.),
            v2: Vector3::new(1., 2., 1.),
        };
        let aabb = Aabb::from_extents(Vector3::new(0., 0., 0.), Vector3::new(2., 2., 2.));

        assert_eq!(intersect_triangle_aabb(&triangle, &aabb), true);
    }

    #[test]
    fn triangle_contained_in_one_face() {
        let triangle = Triangle {
            v0: Vector3::new(1., 1., 2.),
            v1: Vector3::new(2., 1., 2.),
            v2: Vector3::new(1., 2., 2.),
        };
        let aabb = Aabb::from_extents(Vector3::new(0., 0., 0.), Vector3::new(2., 2., 2.));

        assert_eq!(intersect_triangle_aabb(&triangle, &aabb), true);
    }

    #[test]
    fn triangle_outside() {
        let triangle = Triangle {
            v0: Vector3::new(10., 10., 10.),
            v1: Vector3::new(11., 10., 10.),
            v2: Vector3::new(10., 11., 10.),
        };
        let aabb = Aabb::from_extents(Vector3::new(0., 0., 0.), Vector3::new(2., 2., 2.));

        assert_eq!(intersect_triangle_aabb(&triangle, &aabb), false);
    }
}
