use nalgebra::Vector3;

use crate::{aabb::Aabb, intersection::RayIntersection, ray::Ray, Geometry};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl Geometry for Sphere {
    fn min(&self) -> Vector3<f32> {
        Vector3::new(
            self.center.x - self.radius,
            self.center.y - self.radius,
            self.center.z - self.radius,
        )
    }

    fn max(&self) -> Vector3<f32> {
        Vector3::new(
            self.center.x + self.radius,
            self.center.y + self.radius,
            self.center.z + self.radius,
        )
    }

    fn intersect_ray(&self, ray: &Ray) -> Option<RayIntersection> {
        let p = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&p);
        let c = p.dot(&p) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        dbg!(t1, t2);
        let t = if t1 <= t2 { t1 } else { t2 };
        Some(RayIntersection { t, u: 0.0, v: 0.0 })
    }

    fn clip_aabb(&self, _: &Aabb) -> Option<Aabb> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_ray_axially_through_center() {
        let sphere = Sphere {
            center: Vector3::new(1.0, 0.0, 0.0),
            radius: 1.0,
        };
        let ray = Ray::between(&Vector3::new(0.0, 0.0, 0.0), &Vector3::new(2.0, 0.0, 0.0));

        let actual = sphere.intersect_ray(&ray);

        assert_eq!(
            actual,
            Some(RayIntersection {
                t: 0.0,
                u: 0.0,
                v: 0.0
            })
        );
    }

    #[test]
    fn intersect_ray_diagonally_through_center() {
        let sphere = Sphere {
            center: Vector3::new(1.0, 1.0, 1.0),
            radius: 1.0,
        };
        let ray = Ray::between(&Vector3::new(0.0, 0.0, 0.0), &Vector3::new(2.0, 2.0, 2.0));

        let actual = sphere.intersect_ray(&ray);

        assert_eq!(
            actual,
            Some(RayIntersection {
                t: 0.21132487,
                u: 0.0,
                v: 0.0
            })
        );
    }

    #[test]
    fn intersect_ray_touching_in_a_point() {
        let sphere = Sphere {
            center: Vector3::new(1.0, 0.0, 0.0),
            radius: 1.0,
        };
        let ray = Ray::between(&Vector3::new(0.0, 1.0, 0.0), &Vector3::new(2.0, 1.0, 0.0));

        let actual = sphere.intersect_ray(&ray);

        assert_eq!(
            actual,
            Some(RayIntersection {
                t: 0.5,
                u: 0.0,
                v: 0.0
            })
        );
    }

    #[test]
    fn intersect_ray_not_intersecting() {
        let sphere = Sphere {
            center: Vector3::new(1.0, 0.0, 0.0),
            radius: 1.0,
        };
        let ray = Ray::between(&Vector3::new(0.0, 2.0, 0.0), &Vector3::new(2.0, 2.0, 0.0));

        let actual = sphere.intersect_ray(&ray);

        assert_eq!(actual, None);
    }
}
