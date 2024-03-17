use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::triangle::Triangle;
use nalgebra::{vector, Vector3};
use smallvec::SmallVec;

use super::aap::{Aap, Axis};

#[derive(Debug, PartialEq)]
pub struct TriangleRayIntersection {
    pub t: f32,
    pub u: f32,
    pub v: f32,
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
    use nalgebra::vector;

    #[test]
    fn through_base_center() {
        let triangle = Triangle {
            v0: vector![0., 0., 0.],
            v1: vector![1., 0., 0.],
            v2: vector![0., 1., 0.],
        };
        let ray = Ray::between(
            &vector![triangle.base_center().x, triangle.base_center().y, -1.],
            &vector![triangle.base_center().x, triangle.base_center().y, 1.],
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
            v0: vector![0., 0., 0.],
            v1: vector![1., 0., 0.],
            v2: vector![0., 1., 0.],
        };
        let ray = Ray::between(
            &vector![triangle.v0.x, triangle.v0.y, -1.],
            &vector![triangle.v0.x, triangle.v0.y, 1.],
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
            v0: vector![0., 0., 0.],
            v1: vector![1., 0., 0.],
            v2: vector![0., 1., 0.],
        };
        let ray = Ray::between(
            &vector![triangle.v1.x, triangle.v1.y, -1.],
            &vector![triangle.v1.x, triangle.v1.y, 1.],
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
            v0: vector![0., 0., 0.],
            v1: vector![1., 0., 0.],
            v2: vector![0., 1., 0.],
        };
        let ray = Ray::between(
            &vector![triangle.v2.x, triangle.v2.y, -1.],
            &vector![triangle.v2.x, triangle.v2.y, 1.],
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
            v0: vector![0., 0., 0.],
            v1: vector![1., 0., 0.],
            v2: vector![0., 1., 0.],
        };
        let intersection_point = triangle.v0 + triangle.edge0() / 2.;
        let ray = Ray::between(
            &vector![intersection_point.x, intersection_point.y, -1.],
            &vector![intersection_point.x, intersection_point.y, 1.],
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
            v0: vector![0., 0., 0.],
            v1: vector![1., 0., 0.],
            v2: vector![0., 1., 0.],
        };
        let intersection_point = triangle.v1 + triangle.edge1() / 2.;
        let ray = Ray::between(
            &vector![intersection_point.x, intersection_point.y, -1.],
            &vector![intersection_point.x, intersection_point.y, 1.],
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
            v0: vector![0., 0., 0.],
            v1: vector![1., 0., 0.],
            v2: vector![0., 1., 0.],
        };
        let intersection_point = triangle.v2 + triangle.edge2() / 2.;
        let ray = Ray::between(
            &vector![intersection_point.x, intersection_point.y, -1.],
            &vector![intersection_point.x, intersection_point.y, 1.],
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
            v0: vector![0., 0., 0.],
            v1: vector![1., 0., 0.],
            v2: vector![0., 1., 0.],
        };
        let ray = Ray::between(
            &vector![triangle.v0.x, triangle.v0.y, 0.],
            &vector![triangle.v1.x, triangle.v1.y, 0.],
        );

        assert_eq!(intersect_triangle_ray(&triangle, &ray), None);
    }

    #[test]
    fn parallel_not_touching() {
        let triangle = Triangle {
            v0: vector![0., 0., 0.],
            v1: vector![1., 0., 0.],
            v2: vector![0., 1., 0.],
        };
        let ray = Ray::between(
            &vector![triangle.v0.x, triangle.v0.y, 1.],
            &vector![triangle.v1.x, triangle.v1.y, 1.],
        );

        assert_eq!(intersect_triangle_ray(&triangle, &ray), None);
    }

    #[test]
    fn almost_parallel_touching() {
        let triangle = Triangle {
            v0: vector![0., 0., 0.],
            v1: vector![1., 0., 0.],
            v2: vector![0., 1., 0.],
        };
        let ray = Ray::between(
            &vector![triangle.v0.x, triangle.v0.y, -0.000001],
            &vector![triangle.v1.x, triangle.v1.y, 0.000001],
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
    let v0 = triangle.v0 - aabb.center;
    let v1 = triangle.v1 - aabb.center;
    let v2 = triangle.v2 - aabb.center;

    let f0 = v1 - v0;
    let f1 = v2 - v1;
    let f2 = v0 - v2;

    const U0: Vector3<f32> = vector![1., 0., 0.];
    const U1: Vector3<f32> = vector![0., 1., 0.];
    const U2: Vector3<f32> = vector![0., 0., 1.];

    let test_axis = |axis: Vector3<f32>| {
        let p0 = v0.dot(&axis);
        let p1 = v1.dot(&axis);
        let p2 = v2.dot(&axis);
        let r = aabb.half_size.x * U0.dot(&axis).abs()
            + aabb.half_size.y * U1.dot(&axis).abs()
            + aabb.half_size.z * U2.dot(&axis).abs();
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
    use nalgebra::vector;

    #[test]
    fn triangle_completely_inside() {
        let triangle = Triangle {
            v0: vector![1., 1., 1.],
            v1: vector![2., 1., 1.],
            v2: vector![1., 2., 1.],
        };
        let aabb = Aabb {
            center: vector![1., 1., 1.],
            half_size: vector![1., 1., 1.],
        };

        assert_eq!(intersect_triangle_aabb(&triangle, &aabb), true);
    }

    #[test]
    fn triangle_contained_in_one_face() {
        let triangle = Triangle {
            v0: vector![1., 1., 2.],
            v1: vector![2., 1., 2.],
            v2: vector![1., 2., 2.],
        };
        let aabb = Aabb {
            center: vector![1., 1., 1.],
            half_size: vector![1., 1., 1.],
        };

        assert_eq!(intersect_triangle_aabb(&triangle, &aabb), true);
    }

    #[test]
    fn triangle_outside() {
        let triangle = Triangle {
            v0: vector![10., 10., 10.],
            v1: vector![11., 10., 10.],
            v2: vector![10., 11., 10.],
        };
        let aabb = Aabb {
            center: vector![1., 1., 1.],
            half_size: vector![1., 1., 1.],
        };

        assert_eq!(intersect_triangle_aabb(&triangle, &aabb), false);
    }
}

pub fn triangles_bounding_box(triangles: &[Triangle]) -> Aabb {
    if triangles.is_empty() {
        return Aabb::empty();
    }
    let mut a = triangles[0].min();
    let mut b = triangles[0].max();
    for triangle in triangles {
        a = a.inf(&triangle.min());
        b = b.sup(&triangle.max());
    }
    Aabb::from_extents(&a, &b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::vector;

    #[test]
    fn test_bounding() {
        let triangles = [
            Triangle {
                v0: vector![1., 1., 0.],
                v1: vector![1., 1., 1.],
                v2: vector![0., 0., 0.],
            },
            Triangle {
                v0: vector![-1., -1., 0.],
                v1: vector![-1., -1., -1.],
                v2: vector![0., 0., 0.],
            },
        ];

        let actual = triangles_bounding_box(&triangles);

        let expected = Aabb {
            center: vector![0., 0., 0.],
            half_size: vector![1., 1., 1.],
        };
        assert_eq!(actual, expected);
    }
}

pub fn intersect_ray_aap(ray: &Ray, plane: &Aap) -> Option<f32> {
    let d = plane.distance - ray.origin[plane.axis];
    let a = plane.axis.as_vector3(d);
    let b = ray.direction;
    let f = a.dot(&b);
    if f == 0.0 {
        // Ray parallel to plane (ray direction vector orthogonal to plane vector).
        return None;
    }
    let t = f / b.dot(&b);
    (0.0..=1.0).contains(&t).then_some(t)
}

#[cfg(test)]
mod tests_intersect_ray_aap {
    use crate::geometry::aap::Axis;

    use super::*;

    #[test]
    fn test_ray_from_origo_to_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::zeros(), &vector![5.0, 0.0, 0.0]);

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(1.0));
    }

    #[test]
    fn test_ray_from_origo_to_beyond_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::zeros(), &vector![10.0, 0.0, 0.0]);

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(0.5));
    }

    #[test]
    fn test_ray_from_origo_to_short_of_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::zeros(), &vector![4.0, 0.0, 0.0]);

        assert_eq!(intersect_ray_aap(&ray, &plane), None);
    }

    #[test]
    fn test_ray_from_just_before_plane_to_beyond_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&vector![4.0, 0.0, 0.0], &vector![6.0, 0.0, 0.0]);

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(0.5));
    }

    #[test]
    fn test_ray_from_just_after_plane_to_before_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&vector![6.0, 0.0, 0.0], &vector![4.0, 0.0, 0.0]);

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(0.5));
    }

    #[test]
    fn test_non_axis_aligned_ray_through_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 2.0,
        };
        let ray = Ray::between(&vector![1.0, 1.0, 0.0], &(vector![3.0, 3.0, 0.0]));

        assert_eq!(intersect_ray_aap(&ray, &plane), Some(0.25));
    }

    #[test]
    fn test_ray_parallel_to_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 2.0,
        };
        let ray = Ray::between(&vector![0.0, 0.0, 0.0], &(vector![0.0, 1.0, 0.0]));

        assert_eq!(intersect_ray_aap(&ray, &plane), None);
    }

    #[test]
    fn test() {
        let plane = Aap {
            axis: Axis::Y,
            distance: 4.0,
        };
        let ray = Ray::between(&vector![12.0, 0.0, 0.0], &vector![6.0, 6.0, 0.0]);

        let actual = intersect_ray_aap(&ray, &plane);

        assert_eq!(actual.map(|t| ray.param(t)), Some(vector![8.0, 4.0, 0.0]));
    }
}

pub fn clip_triangle_aabb(triangle: &Triangle, aabb: &Aabb) -> SmallVec<[Vector3<f32>; 18]> {
    let mut points = SmallVec::<[Vector3<f32>; 18]>::new();
    let aabb_min = aabb.min();
    let aabb_max = aabb.max();

    let mut test_against_plane = |ray: &Ray, axis, extreme: &Vector3<f32>| {
        let plane = Aap {
            axis,
            distance: extreme[axis],
        };
        match intersect_ray_aap(ray, &plane) {
            Some(param) => {
                let point = ray.param(param);
                if point >= aabb_min && point <= aabb_max {
                    points.push(point);
                }
            }
            _ => (),
        }
    };

    let mut test_axes = |ray| {
        test_against_plane(&ray, Axis::X, &aabb_min);
        test_against_plane(&ray, Axis::X, &aabb_max);
        test_against_plane(&ray, Axis::Y, &aabb_min);
        test_against_plane(&ray, Axis::Y, &aabb_max);
        test_against_plane(&ray, Axis::Z, &aabb_min);
        test_against_plane(&ray, Axis::Z, &aabb_max);
    };

    test_axes(triangle.edge0_ray());
    test_axes(triangle.edge1_ray());
    test_axes(triangle.edge2_ray());

    // TODO: Avoid having to dedup by special-casing intersection.
    points.dedup();
    points
}

#[cfg(test)]
mod tests_clip_triangle_aabb {
    use super::*;
    use smallvec::smallvec;

    #[test]
    pub fn triangle_completely_enclosed_in_box() {
        let triangle = Triangle {
            v0: vector![1.0, 1.0, 1.0],
            v1: vector![2.0, 1.0, 1.0],
            v2: vector![2.0, 2.0, 1.0],
        };
        let aabb = Aabb::from_extents(&vector![0.0, 0.0, 0.0], &vector![2.0, 2.0, 2.0]);

        let actual = clip_triangle_aabb(&triangle, &aabb);

        let expected: SmallVec<[_; 3]> = smallvec![triangle.v1, triangle.v2, triangle.v0];
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn triangle_above_box() {
        let triangle = Triangle {
            v0: vector![0.0, 2.0, 0.0],
            v1: vector![1.0, 2.0, 0.0],
            v2: vector![1.0, 2.0, 1.0],
        };
        let aabb = Aabb::from_extents(&vector![0.0, 0.0, 0.0], &vector![1.0, 1.0, 1.0]);

        let actual = clip_triangle_aabb(&triangle, &aabb);

        let expected: SmallVec<[Vector3<f32>; 3]> = smallvec![];
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn triangle_below_box() {
        let triangle = Triangle {
            v0: vector![0.0, -1.0, 0.0],
            v1: vector![1.0, -1.0, 0.0],
            v2: vector![1.0, -1.0, 1.0],
        };
        let aabb = Aabb::from_extents(&vector![0.0, 0.0, 0.0], &vector![1.0, 1.0, 1.0]);

        let actual = clip_triangle_aabb(&triangle, &aabb);

        let expected: SmallVec<[Vector3<f32>; 3]> = smallvec![];
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn triangle_in_zplane_with_all_edges_intersecting_box_sides() {
        let triangle = Triangle {
            v0: vector![0.0, 0.0, 0.0],
            v1: vector![12.0, 0.0, 0.0],
            v2: vector![6.0, 6.0, 0.0],
        };
        let aabb = Aabb::from_extents(&vector![2.0, -1.0, 0.0], &vector![10.0, 4.0, 0.0]);

        let actual = clip_triangle_aabb(&triangle, &aabb);

        let expected: SmallVec<[Vector3<f32>; 3]> = smallvec![
            vector![2.0, 0.0, 0.0],
            vector![10.0, 0.0, 0.0],
            vector![10.0, 2.0, 0.0],
            vector![8.0, 4.0, 0.0],
            vector![4.0, 4.0, 0.0],
            vector![2.0, 2.0, 0.0],
        ];
        assert_eq!(actual, expected);
    }
}
