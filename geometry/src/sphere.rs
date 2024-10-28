use glam::Vec3;

use crate::{intersection::RayIntersection, ray::Ray};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    pub fn min(&self) -> Vec3 {
        Vec3::new(
            self.center.x - self.radius,
            self.center.y - self.radius,
            self.center.z - self.radius,
        )
    }

    pub fn max(&self) -> Vec3 {
        Vec3::new(
            self.center.x + self.radius,
            self.center.y + self.radius,
            self.center.z + self.radius,
        )
    }

    pub fn normal_point(&self, point: &Vec3) -> Vec3 {
        debug_assert!((point - self.center).length() == self.radius);
        (point - self.center).normalize()
    }

    pub fn normal_parametric(&self, theta: f32, phi: f32) -> Vec3 {
        Vec3::new(phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos())
    }

    pub fn intersect_ray(&self, ray: &Ray) -> Option<RayIntersection> {
        let p = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(p);
        let c = p.dot(p) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t = if t1 <= t2 { t1 } else { t2 };

        let normal = (p + t * ray.direction) / self.radius;
        Some(RayIntersection {
            t,
            u: 0.0,
            v: 0.0,
            normal,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_ray_axially_through_center() {
        let sphere = Sphere {
            center: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
        };
        let ray = Ray::between(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0));

        let actual = sphere.intersect_ray(&ray);

        assert_eq!(
            actual,
            Some(RayIntersection {
                t: 0.0,
                u: 0.0,
                v: 0.0,
                normal: Vec3::new(-1.0, 0.0, 0.0),
            })
        );
    }

    #[test]
    fn intersect_ray_diagonally_through_center() {
        let sphere = Sphere {
            center: Vec3::new(1.0, 1.0, 1.0),
            radius: 1.0,
        };
        let ray = Ray::between(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 2.0, 2.0));

        let actual = sphere.intersect_ray(&ray);

        assert_eq!(
            actual,
            Some(RayIntersection {
                t: 0.21132487,
                u: 0.0,
                v: 0.0,
                normal: Vec3::new(-0.57735026, -0.57735026, -0.57735026),
            })
        );
    }

    #[test]
    fn intersect_ray_touching_in_a_point() {
        let sphere = Sphere {
            center: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
        };
        let ray = Ray::between(Vec3::new(0.0, 1.0, 0.0), Vec3::new(2.0, 1.0, 0.0));

        let actual = sphere.intersect_ray(&ray);

        assert_eq!(
            actual,
            Some(RayIntersection {
                t: 0.5,
                u: 0.0,
                v: 0.0,
                normal: Vec3::new(0.0, 1.0, 0.0),
            })
        );
    }

    #[test]
    fn intersect_ray_not_intersecting() {
        let sphere = Sphere {
            center: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
        };
        let ray = Ray::between(Vec3::new(0.0, 2.0, 0.0), Vec3::new(2.0, 2.0, 0.0));

        let actual = sphere.intersect_ray(&ray);

        assert_eq!(actual, None);
    }
}
