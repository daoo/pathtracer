use crate::{
    aabb::Aabb, axial_triangle::AxiallyAlignedTriangle, intersection::RayIntersection, ray::Ray,
    triangle::Triangle, Geometry,
};
use nalgebra::Vector3;

#[derive(Clone, Debug, PartialEq)]
pub enum Geometric {
    Triangle(Triangle),
    AxiallyAlignedTriangle(AxiallyAlignedTriangle),
}

impl Geometry for Geometric {
    #[inline]
    fn min(&self) -> Vector3<f32> {
        match self {
            Geometric::Triangle(t) => t.min(),
            Geometric::AxiallyAlignedTriangle(t) => t.min(),
        }
    }

    #[inline]
    fn max(&self) -> Vector3<f32> {
        match self {
            Geometric::Triangle(t) => t.max(),
            Geometric::AxiallyAlignedTriangle(t) => t.max(),
        }
    }

    #[inline]
    fn intersect_ray(&self, ray: &Ray) -> Option<RayIntersection> {
        match self {
            Geometric::Triangle(t) => t.intersect_ray(ray),
            Geometric::AxiallyAlignedTriangle(t) => t.intersect_ray(ray),
        }
    }

    #[inline]
    fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb> {
        match self {
            Geometric::Triangle(t) => t.clip_aabb(aabb),
            Geometric::AxiallyAlignedTriangle(t) => t.clip_aabb(aabb),
        }
    }
}

impl From<Triangle> for Geometric {
    fn from(value: Triangle) -> Self {
        value
            .as_axially_aligned()
            .map(Geometric::AxiallyAlignedTriangle)
            .unwrap_or(Geometric::Triangle(value))
    }
}
