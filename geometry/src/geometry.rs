use glam::Vec3;

use crate::{
    aabb::Aabb, axial_triangle::AxiallyAlignedTriangle, intersection::RayIntersection, ray::Ray,
    triangle::Triangle,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Geometry {
    Triangle(Triangle),
    AxiallyAlignedTriangle(AxiallyAlignedTriangle),
}

impl Geometry {
    #[inline]
    pub fn min(&self) -> Vec3 {
        match self {
            Geometry::Triangle(t) => t.min(),
            Geometry::AxiallyAlignedTriangle(t) => t.min(),
        }
    }

    #[inline]
    pub fn max(&self) -> Vec3 {
        match self {
            Geometry::Triangle(t) => t.max(),
            Geometry::AxiallyAlignedTriangle(t) => t.max(),
        }
    }

    #[inline]
    pub fn intersect_ray(&self, ray: &Ray) -> Option<RayIntersection> {
        match self {
            Geometry::Triangle(t) => t.intersect_ray(ray),
            Geometry::AxiallyAlignedTriangle(t) => t.intersect_ray(ray),
        }
    }

    #[inline]
    pub fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb> {
        match self {
            Geometry::Triangle(t) => t.clip_aabb(aabb),
            Geometry::AxiallyAlignedTriangle(t) => t.clip_aabb(aabb),
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
