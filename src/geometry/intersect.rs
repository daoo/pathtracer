use crate::geometry::ray::Ray;
use crate::geometry::triangle::Triangle;
use nalgebra::Vector2;
use nalgebra::Vector3;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TriangleRayIntersection<'a, 'b> {
    triangle: &'a Triangle,
    ray: &'b Ray,
    t: f32,
    u: f32,
    v: f32,
}

pub fn intersect<'a, 'b>(triangle: &'a Triangle, ray: &'b Ray) -> Option<TriangleRayIntersection<'a, 'b>> {
    let epsilon = 0.00001;

    let e1 = &triangle.v1 - &triangle.v0;
    let e2 = &triangle.v2 - &triangle.v0;
    let q = ray.direction.cross(&e2);

    let a = e1.dot(&q);
    if a > -epsilon && a < epsilon {
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
    Some(TriangleRayIntersection {triangle: &triangle, ray: &ray, t, u, v})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let triangle = Triangle {
            v0: Vector3::new(-1.0, -1.0, 0.0),
            v1: Vector3::new(1.0, -1.0, 0.0),
            v2: Vector3::new(0.0, 2.0, 0.0),
            n0: Vector3::zeros(),
            n1: Vector3::zeros(),
            n2: Vector3::zeros(),
            uv0: Vector2::zeros(),
            uv1: Vector2::zeros(),
            uv2: Vector2::zeros(),
        };
        let ray = Ray::from(&Vector3::new(0.0, 0.0, -1.0), &Vector3::new(0.0, 0.0, 1.0));

        let expected = TriangleRayIntersection {
            triangle: &triangle,
            ray: &ray,
            t: 0.5,
            u: 0.33333334,
            v: 0.33333334,
        };
        assert_eq!(intersect(&triangle, &ray), Some(expected));
    }
}
