use aabb::Aabb;
use intersection::RayIntersection;
use nalgebra::Vector3;
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
pub mod triangle;

pub trait Geometry {
    fn min(&self) -> Vector3<f32>;
    fn max(&self) -> Vector3<f32>;

    fn intersect_ray(&self, ray: &Ray) -> Option<RayIntersection>;
    fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb>;
}
