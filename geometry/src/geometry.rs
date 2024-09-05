use glam::Vec3;

use crate::{
    aabb::Aabb, axial_triangle::AxiallyAlignedTriangle, intersection::RayIntersection, ray::Ray,
    sphere::Sphere, triangle::Triangle,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Geometry {
    Triangle(Triangle),
    AxiallyAlignedTriangle(AxiallyAlignedTriangle),
    Sphere(Sphere),
}

impl Geometry {
    #[inline]
    pub fn min(&self) -> Vec3 {
        match self {
            Geometry::Triangle(g) => g.min(),
            Geometry::AxiallyAlignedTriangle(g) => g.min(),
            Geometry::Sphere(s) => s.min(),
        }
    }

    #[inline]
    pub fn max(&self) -> Vec3 {
        match self {
            Geometry::Triangle(g) => g.max(),
            Geometry::AxiallyAlignedTriangle(g) => g.max(),
            Geometry::Sphere(s) => s.max(),
        }
    }

    #[inline]
    pub fn intersect_ray(&self, ray: &Ray) -> Option<RayIntersection> {
        match self {
            Geometry::Triangle(g) => g.intersect_ray(ray),
            Geometry::AxiallyAlignedTriangle(g) => g.intersect_ray(ray),
            Geometry::Sphere(s) => s.intersect_ray(ray),
        }
    }

    #[inline]
    pub fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb> {
        match self {
            Geometry::Triangle(g) => g.clip_aabb(aabb),
            Geometry::AxiallyAlignedTriangle(g) => g.clip_aabb(aabb),
            Geometry::Sphere(_) => todo!(),
        }
    }
}

impl From<Triangle> for Geometry {
    #[inline]
    fn from(value: Triangle) -> Self {
        value
            .as_axially_aligned()
            .map(Geometry::AxiallyAlignedTriangle)
            .unwrap_or(Geometry::Triangle(value))
    }
}

impl From<Sphere> for Geometry {
    #[inline]
    fn from(value: Sphere) -> Self {
        Geometry::Sphere(value)
    }
}
