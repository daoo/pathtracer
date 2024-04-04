use nalgebra::Vector3;

use super::{aabb::Aabb, aap::Aap, ray::Ray, triangle::Triangle};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TriangleRayIntersection {
    pub t: f32,
    pub u: f32,
    pub v: f32,
}

impl TriangleRayIntersection {
    pub fn new(t: f32, u: f32, v: f32) -> Self {
        TriangleRayIntersection { t, u, v }
    }
}

pub fn intersect_triangle_ray(triangle: &Triangle, ray: &Ray) -> Option<TriangleRayIntersection> {
    let b0 = triangle.base0();
    let b1 = triangle.base1();
    let q = ray.direction.cross(&b1);

    let a = b0.dot(&q);
    if a == 0. {
        return None;
    }

    let s = ray.origin - triangle.v0;
    let f = 1. / a;
    let u = f * s.dot(&q);
    if !(0. ..=1.).contains(&u) {
        return None;
    }

    let r = s.cross(&b0);
    let v = f * ray.direction.dot(&r);
    if v < 0. || (u + v) > 1. {
        return None;
    }

    let t = f * b1.dot(&r);
    Some(TriangleRayIntersection { t, u, v })
}

#[cfg(test)]
mod tests_intersect_triangle_ray {
    use super::*;

    #[test]
    fn through_base_center() {
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
            intersect_triangle_ray(&triangle, &ray),
            Some(TriangleRayIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.5
            })
        );
    }

    #[test]
    fn through_v0() {
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
            intersect_triangle_ray(&triangle, &ray),
            Some(TriangleRayIntersection {
                t: 0.5,
                u: 0.,
                v: 0.
            })
        );
    }

    #[test]
    fn through_v1() {
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
            intersect_triangle_ray(&triangle, &ray),
            Some(TriangleRayIntersection {
                t: 0.5,
                u: 1.,
                v: 0.
            })
        );
    }

    #[test]
    fn through_v2() {
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
            intersect_triangle_ray(&triangle, &ray),
            Some(TriangleRayIntersection {
                t: 0.5,
                u: 0.,
                v: 1.
            })
        );
    }

    #[test]
    fn through_edge0() {
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
            intersect_triangle_ray(&triangle, &ray),
            Some(TriangleRayIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.
            })
        );
    }

    #[test]
    fn through_edge1() {
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
            intersect_triangle_ray(&triangle, &ray),
            Some(TriangleRayIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.5
            })
        );
    }

    #[test]
    fn through_edge2() {
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
            intersect_triangle_ray(&triangle, &ray),
            Some(TriangleRayIntersection {
                t: 0.5,
                u: 0.,
                v: 0.5
            })
        );
    }

    #[test]
    fn parallel_touching() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            &Vector3::new(triangle.v0.x, triangle.v0.y, 0.),
            &Vector3::new(triangle.v1.x, triangle.v1.y, 0.),
        );

        assert_eq!(intersect_triangle_ray(&triangle, &ray), None);
    }

    #[test]
    fn parallel_not_touching() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let ray = Ray::between(
            &Vector3::new(triangle.v0.x, triangle.v0.y, 1.),
            &Vector3::new(triangle.v1.x, triangle.v1.y, 1.),
        );

        assert_eq!(intersect_triangle_ray(&triangle, &ray), None);
    }

    #[test]
    fn almost_parallel_touching() {
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
            intersect_triangle_ray(&triangle, &ray),
            Some(TriangleRayIntersection {
                t: 0.5,
                u: 0.5,
                v: 0.
            })
        );
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

pub fn intersect_ray_aap(ray: &Ray, plane: &Aap) -> Option<f32> {
    let normal = plane.axis.as_vector3(1.0);
    let denom = normal.dot(&ray.direction);
    if denom == 0.0 {
        // Ray parallel to plane (ray direction vector orthogonal to plane vector).
        return None;
    }
    let t = (plane.axis.as_vector3(plane.distance) - ray.origin).dot(&normal) / denom;
    (0.0..=1.0).contains(&t).then_some(t)
}

#[cfg(test)]
mod tests_intersect_ray_aap {
    use crate::axis::Axis;

    use super::*;

    #[test]
    fn test_ray_from_origo_to_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::zeros(), &Vector3::new(5.0, 0.0, 0.0));

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(1.0));
    }

    #[test]
    fn test_ray_from_origo_to_beyond_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::zeros(), &Vector3::new(10.0, 0.0, 0.0));

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(0.5));
    }

    #[test]
    fn test_ray_from_origo_to_short_of_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::zeros(), &Vector3::new(4.0, 0.0, 0.0));

        assert_eq!(intersect_ray_aap(&ray, &plane), None);
    }

    #[test]
    fn test_ray_from_just_before_plane_to_beyond_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::new(4.0, 0.0, 0.0), &Vector3::new(6.0, 0.0, 0.0));

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(0.5));
    }

    #[test]
    fn test_ray_from_just_after_plane_to_before_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::new(6.0, 0.0, 0.0), &Vector3::new(4.0, 0.0, 0.0));

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(0.5));
    }

    #[test]
    fn test_non_axis_aligned_ray_through_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 2.0,
        };
        let ray = Ray::between(&Vector3::new(1.0, 1.0, 0.0), &(Vector3::new(3.0, 3.0, 0.0)));

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(0.5));
    }

    #[test]
    fn test_non_axis_aligned_ray_with_positive_direction_through_plane() {
        let plane = Aap {
            axis: Axis::Y,
            distance: 2.0,
        };
        let ray = Ray::between(&Vector3::new(1.0, 1.0, 0.0), &Vector3::new(3.0, 3.0, 0.0));

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(0.5));
    }

    #[test]
    fn test_non_axis_aligned_ray_with_negative_direction_through_plane() {
        let plane = Aap {
            axis: Axis::Y,
            distance: 2.0,
        };
        let ray = Ray::between(&Vector3::new(3.0, 1.0, 0.0), &Vector3::new(1.0, 3.0, 0.0));

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(0.5));
    }

    #[test]
    fn test_ray_parallel_to_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 2.0,
        };
        let ray = Ray::between(&Vector3::new(0.0, 0.0, 0.0), &(Vector3::new(0.0, 1.0, 0.0)));

        assert_eq!(intersect_ray_aap(&ray, &plane), None);
    }
}
