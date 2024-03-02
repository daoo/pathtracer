use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::triangle::Triangle;
use nalgebra::Vector3;

#[derive(Debug, PartialEq)]
pub struct TriangleRayIntersection {
    pub t: f32,
    pub u: f32,
    pub v: f32,
}

pub fn intersect(triangle: &Triangle, ray: &Ray) -> Option<TriangleRayIntersection> {
    let e1 = triangle.e1();
    let e2 = triangle.e2();
    let q = ray.direction.cross(&e2);

    let a = e1.dot(&q);
    if a == 0.0 {
        return None
    }

    let s = &ray.origin - &triangle.v0;
    let f = 1.0 / a;
    let u = f * s.dot(&q);
    if u < 0.0 || u > 1.0 {
        return None
    }

    let r = s.cross(&e1);
    let v = f * ray.direction.dot(&r);
    if v < 0.0 || (u + v) > 1.0 {
        return None
    }

    let t = f * e2.dot(&r);
    Some(TriangleRayIntersection{t, u, v})
}

pub fn find_closest_intersection(triangles: &[Triangle], ray: &Ray, mint: f32, maxt: f32) -> Option<TriangleRayIntersection> {
    let mut closest: Option<TriangleRayIntersection> = None;
    let mut maxt = maxt;
    for triangle in triangles {
        closest = match intersect(triangle, ray) {
            Some(intersection) if intersection.t >= mint && intersection.t <= maxt => {
                maxt = intersection.t;
                Some(intersection)
            },
            _ => closest
        };
    }
    closest
}

pub fn intersect_triangle_aabb(triangle: &Triangle, aabb: &Aabb) -> bool {
    let v0 = &triangle.v0 - &aabb.center;
    let v1 = &triangle.v1 - &aabb.center;
    let v2 = &triangle.v2 - &aabb.center;

    let f0 = &v1 - &v0;
    let f1 = &v2 - &v1;
    let f2 = &v0 - &v2;

    let u0 = Vector3::new(1.0, 0.0, 0.0);
    let u1 = Vector3::new(0.0, 1.0, 0.0);
    let u2 = Vector3::new(0.0, 0.0, 1.0);

    let test_axis = |axis: &Vector3<f32>| {
        let p0 = triangle.v0.dot(&axis);
        let p1 = triangle.v1.dot(&axis);
        let p2 = triangle.v2.dot(&axis);
        let r = aabb.half_size.x * u0.dot(&axis).abs() +
            aabb.half_size.y * u1.dot(&axis).abs() +
            aabb.half_size.z * u2.dot(&axis).abs();
        (-p0.max(p1.max(p2))).max(p0.min(p1.min(p2))) > r
    };

    if test_axis(&u0.cross(&f0)) { return false; }
    if test_axis(&u0.cross(&f1)) { return false; }
    if test_axis(&u0.cross(&f2)) { return false; }
    if test_axis(&u1.cross(&f0)) { return false; }
    if test_axis(&u1.cross(&f1)) { return false; }
    if test_axis(&u1.cross(&f2)) { return false; }
    if test_axis(&u2.cross(&f0)) { return false; }
    if test_axis(&u2.cross(&f1)) { return false; }
    if test_axis(&u2.cross(&f2)) { return false; }

    if test_axis(&u0) { return false; }
    if test_axis(&u1) { return false; }
    if test_axis(&u2) { return false; }

    let triangle_normal = f0.cross(&f1);
    if test_axis(&triangle_normal) { return false; }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector2;
    use nalgebra::Vector3;

    fn triangle(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>) -> Triangle {
        Triangle {
            v0,
            v1,
            v2,
            n0: Vector3::zeros(),
            n1: Vector3::zeros(),
            n2: Vector3::zeros(),
            uv0: Vector2::zeros(),
            uv1: Vector2::zeros(),
            uv2: Vector2::zeros(),
        }
    }

    #[test]
    fn test_intersect_center() {
        let triangle = triangle(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let ray = Ray::between(
            &Vector3::new(triangle.center().x, triangle.center().y, -1.0),
            &Vector3::new(triangle.center().x, triangle.center().y, 1.0));

        assert_eq!(intersect(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 0.5, v: 0.5}));
    }

    #[test]
    fn test_intersect_through_v0() {
        let triangle = triangle(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let ray = Ray::between(
            &Vector3::new(triangle.v0.x, triangle.v0.y, -1.0),
            &Vector3::new(triangle.v0.x, triangle.v0.y, 1.0));

        assert_eq!(intersect(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 0.0, v: 0.0}));
    }

    #[test]
    fn test_intersect_through_v1() {
        let triangle = triangle(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let ray = Ray::between(
            &Vector3::new(triangle.v1.x, triangle.v1.y, -1.0),
            &Vector3::new(triangle.v1.x, triangle.v1.y, 1.0));

        assert_eq!(intersect(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 1.0, v: 0.0}));
    }

    #[test]
    fn test_intersect_through_v2() {
        let triangle = triangle(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let ray = Ray::between(
            &Vector3::new(triangle.v2.x, triangle.v2.y, -1.0),
            &Vector3::new(triangle.v2.x, triangle.v2.y, 1.0));

        assert_eq!(intersect(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 0.0, v: 1.0}));
    }

    #[test]
    fn test_intersect_parallel_touching() {
        let triangle = triangle(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let ray = Ray::between(
            &Vector3::new(triangle.v0.x, triangle.v0.y, 0.0),
            &Vector3::new(triangle.v1.x, triangle.v1.y, 0.0));

        assert_eq!(intersect(&triangle, &ray), None);
    }

    #[test]
    fn test_intersect_parallel_not_touching() {
        let triangle = triangle(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let ray = Ray::between(
            &Vector3::new(triangle.v0.x, triangle.v0.y, 1.0),
            &Vector3::new(triangle.v1.x, triangle.v1.y, 1.0));

        assert_eq!(intersect(&triangle, &ray), None);
    }

    #[test]
    fn test_intersect_almost_parallel_touching() {
        let triangle = triangle(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let ray = Ray::between(
            &Vector3::new(triangle.v0.x, triangle.v0.y, -0.000001),
            &Vector3::new(triangle.v1.x, triangle.v1.y, 0.000001));

        assert_eq!(intersect(&triangle, &ray), Some(TriangleRayIntersection{t: 0.5, u: 0.5, v: 0.0}));
    }

    #[test]
    fn test_intersect_triangle_aabb1() {
        let triangle = triangle(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let aabb = Aabb { center: Vector3::new(0.0, 0.0, 0.0), half_size: Vector3::new(0.5, 0.5, 0.5) };

        assert_eq!(intersect_triangle_aabb(&triangle, &aabb), true);
    }

    #[test]
    fn test_intersect_triangle_aabb2() {
        let triangle = triangle(
            Vector3::new(10.0, 10.0, 10.0),
            Vector3::new(11.0, 10.0, 10.0),
            Vector3::new(10.0, 11.0, 10.0),
        );
        let aabb = Aabb { center: Vector3::new(0.0, 0.0, 0.0), half_size: Vector3::new(0.5, 0.5, 0.5) };

        assert_eq!(intersect_triangle_aabb(&triangle, &aabb), false);
    }
}
