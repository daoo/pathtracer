use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::triangle::Triangle;
use nalgebra::{vector, Vector3};

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
        return None
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
        return None
    }

    let t = f * b1.dot(&r);
    Some(TriangleRayIntersection{t, u, v})
}

#[cfg(test)]
mod tests_intersect_triangle_ray {
    use super::*;
    use nalgebra::vector;

    #[test]
    fn through_base_center() {
        let triangle = Triangle{ v0: vector![0., 0., 0.], v1: vector![1., 0., 0.], v2: vector![0., 1., 0.] };
        let ray = Ray::between(
            &vector![triangle.base_center().x, triangle.base_center().y, -1.],
            &vector![triangle.base_center().x, triangle.base_center().y, 1.]);

        assert_eq!(intersect_triangle_ray(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 0.5, v: 0.5}));
    }

    #[test]
    fn through_v0() {
        let triangle = Triangle{ v0: vector![0., 0., 0.], v1: vector![1., 0., 0.], v2: vector![0., 1., 0.] };
        let ray = Ray::between(
            &vector![triangle.v0.x, triangle.v0.y, -1.],
            &vector![triangle.v0.x, triangle.v0.y, 1.]);

        assert_eq!(intersect_triangle_ray(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 0., v: 0.}));
    }

    #[test]
    fn through_v1() {
        let triangle = Triangle{ v0: vector![0., 0., 0.], v1: vector![1., 0., 0.], v2: vector![0., 1., 0.] };
        let ray = Ray::between(
            &vector![triangle.v1.x, triangle.v1.y, -1.],
            &vector![triangle.v1.x, triangle.v1.y, 1.]);

        assert_eq!(intersect_triangle_ray(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 1., v: 0.}));
    }

    #[test]
    fn through_v2() {
        let triangle = Triangle{ v0: vector![0., 0., 0.], v1: vector![1., 0., 0.], v2: vector![0., 1., 0.] };
        let ray = Ray::between(
            &vector![triangle.v2.x, triangle.v2.y, -1.],
            &vector![triangle.v2.x, triangle.v2.y, 1.]);

        assert_eq!(intersect_triangle_ray(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 0., v: 1.}));
    }

    #[test]
    fn through_edge0() {
        let triangle = Triangle{ v0: vector![0., 0., 0.], v1: vector![1., 0., 0.], v2: vector![0., 1., 0.] };
        let intersection_point = triangle.v0 + triangle.edge0() / 2.;
        let ray = Ray::between(
            &vector![intersection_point.x, intersection_point.y, -1.],
            &vector![intersection_point.x, intersection_point.y, 1.]);

        assert_eq!(intersect_triangle_ray(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 0.5, v: 0.}));
    }

    #[test]
    fn through_edge1() {
        let triangle = Triangle{ v0: vector![0., 0., 0.], v1: vector![1., 0., 0.], v2: vector![0., 1., 0.] };
        let intersection_point = triangle.v1 + triangle.edge1() / 2.;
        let ray = Ray::between(
            &vector![intersection_point.x, intersection_point.y, -1.],
            &vector![intersection_point.x, intersection_point.y, 1.]);

        assert_eq!(intersect_triangle_ray(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 0.5, v: 0.5}));
    }

    #[test]
    fn through_edge2() {
        let triangle = Triangle{ v0: vector![0., 0., 0.], v1: vector![1., 0., 0.], v2: vector![0., 1., 0.] };
        let intersection_point = triangle.v2 + triangle.edge2() / 2.;
        let ray = Ray::between(
            &vector![intersection_point.x, intersection_point.y, -1.],
            &vector![intersection_point.x, intersection_point.y, 1.]);

        assert_eq!(intersect_triangle_ray(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 0., v: 0.5}));
    }

    #[test]
    fn parallel_touching() {
        let triangle = Triangle{ v0: vector![0., 0., 0.], v1: vector![1., 0., 0.], v2: vector![0., 1., 0.] };
        let ray = Ray::between(
            &vector![triangle.v0.x, triangle.v0.y, 0.],
            &vector![triangle.v1.x, triangle.v1.y, 0.]);

        assert_eq!(intersect_triangle_ray(&triangle, &ray), None);
    }

    #[test]
    fn parallel_not_touching() {
        let triangle = Triangle{ v0: vector![0., 0., 0.], v1: vector![1., 0., 0.], v2: vector![0., 1., 0.] };
        let ray = Ray::between(
            &vector![triangle.v0.x, triangle.v0.y, 1.],
            &vector![triangle.v1.x, triangle.v1.y, 1.]);

        assert_eq!(intersect_triangle_ray(&triangle, &ray), None);
    }

    #[test]
    fn almost_parallel_touching() {
        let triangle = Triangle{ v0: vector![0., 0., 0.], v1: vector![1., 0., 0.], v2: vector![0., 1., 0.] };
        let ray = Ray::between(
            &vector![triangle.v0.x, triangle.v0.y, -0.000001],
            &vector![triangle.v1.x, triangle.v1.y, 0.000001]);

        assert_eq!(intersect_triangle_ray(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 0.5, v: 0.}));
    }
}

pub fn intersect_closest_triangle_ray(triangles: &[Triangle], ray: &Ray, tmin: f32, tmax: f32) -> Option<(usize, TriangleRayIntersection)> {
    debug_assert!(tmin < tmax);
    let mut closest: Option<(usize, TriangleRayIntersection)> = None;
    let t1 = tmin;
    let mut t2 = tmax;
    for (i, triangle) in triangles.iter().enumerate() {
        closest = match intersect_triangle_ray(triangle, ray) {
            Some(intersection) if intersection.t >= t1 && intersection.t <= t2 => {
                t2 = intersection.t;
                Some((i, intersection))
            },
            _ => closest
        };
    }
    closest
}

pub fn intersect_triangle_aabb(triangle: &Triangle, aabb: &Aabb) -> bool {
    let v0 = triangle.v0 - aabb.center;
    let v1 = triangle.v1 - aabb.center;
    let v2 = triangle.v2 - aabb.center;

    let f0 = v1 - v0;
    let f1 = v2 - v1;
    let f2 = v0 - v2;

    let u0 = vector![1., 0., 0.];
    let u1 = vector![0., 1., 0.];
    let u2 = vector![0., 0., 1.];

    let test_axis = |axis: Vector3<f32>| {
        let p0 = v0.dot(&axis);
        let p1 = v1.dot(&axis);
        let p2 = v2.dot(&axis);
        let r = aabb.half_size.x * u0.dot(&axis).abs() +
            aabb.half_size.y * u1.dot(&axis).abs() +
            aabb.half_size.z * u2.dot(&axis).abs();
        (-p0.max(p1.max(p2))).max(p0.min(p1.min(p2))) > r
    };

    if test_axis(u0.cross(&f0)) { return false; }
    if test_axis(u0.cross(&f1)) { return false; }
    if test_axis(u0.cross(&f2)) { return false; }
    if test_axis(u1.cross(&f0)) { return false; }
    if test_axis(u1.cross(&f1)) { return false; }
    if test_axis(u1.cross(&f2)) { return false; }
    if test_axis(u2.cross(&f0)) { return false; }
    if test_axis(u2.cross(&f1)) { return false; }
    if test_axis(u2.cross(&f2)) { return false; }

    if test_axis(u0) { return false; }
    if test_axis(u1) { return false; }
    if test_axis(u2) { return false; }

    let triangle_normal = f0.cross(&f1);
    if test_axis(triangle_normal) { return false; }

    true
}

#[cfg(test)]
mod tests_intersect_triangle_aabb {
    use super::*;
    use nalgebra::vector;

    #[test]
    fn triangle_completely_inside() {
        let triangle = Triangle{ v0: vector![1., 1., 1.], v1: vector![2., 1., 1.], v2: vector![1., 2., 1.] };
        let aabb = Aabb { center: vector![1., 1., 1.], half_size: vector![1., 1., 1.] };

        assert_eq!(intersect_triangle_aabb(&triangle, &aabb), true);
    }

    #[test]
    fn triangle_contained_in_one_face() {
        let triangle = Triangle{ v0: vector![1., 1., 2.], v1: vector![2., 1., 2.], v2: vector![1., 2., 2.] };
        let aabb = Aabb { center: vector![1., 1., 1.], half_size: vector![1., 1., 1.] };

        assert_eq!(intersect_triangle_aabb(&triangle, &aabb), true);
    }

    #[test]
    fn triangle_outside() {
        let triangle = Triangle{ v0: vector![10., 10., 10.], v1: vector![11., 10., 10.], v2: vector![10., 11., 10.] };
        let aabb = Aabb { center: vector![1., 1., 1.], half_size: vector![1., 1., 1.] };

        assert_eq!(intersect_triangle_aabb(&triangle, &aabb), false);
    }

//     #[test]
//     fn triangle_that_should_not_fail() {
//         let right_aabb = Aabb { center: vector![-0.41130003, -0.767225, 0.5476], half_size: vector![0.6887, 0.23302495, 0.5524] };
//         let wrong_triangle = Triangle { v0: vector![0.2774, -0.5342, -0.0028], v1: vector![0.7444, -0.5342, -0.0028], v2: vector![0.7444, -1.0012, -0.0028] };

//         assert_eq!(intersect_triangle_aabb(&wrong_triangle, &right_aabb), true);
//     }
}

pub fn triangles_bounding_box(triangles: &[Triangle]) -> Aabb {
    if triangles.is_empty() {
        return Aabb::empty()
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
            Triangle{ v0: vector![1., 1., 0.], v1: vector![1., 1., 1.], v2: vector![0., 0., 0.] },
            Triangle{ v0: vector![-1., -1., 0.], v1: vector![-1., -1., -1.], v2: vector![0., 0., 0.] },
        ];

        let actual = triangles_bounding_box(&triangles);

        let expected = Aabb {
            center: vector![0., 0., 0.],
            half_size: vector![1., 1., 1.]
        };
        assert_eq!(actual, expected);
    }
}
