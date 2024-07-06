use aabb::Aabb;
use glam::Vec3;
use intersection::RayIntersection;
use ray::Ray;

pub mod aabb;
pub mod aap;
pub mod axial_triangle;
pub mod axis;
pub mod bound;
pub mod clip;
pub mod geometric;
pub mod intersection;
pub mod ray;
pub mod sphere;
pub mod triangle;

pub trait Geometry {
    fn min(&self) -> Vec3;
    fn max(&self) -> Vec3;

    fn intersect_ray(&self, ray: &Ray) -> Option<RayIntersection>;
    fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb>;
}
