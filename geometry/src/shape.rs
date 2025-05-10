use glam::Vec3;

use crate::{
    aabb::Aabb,
    axial_triangle::AxiallyAlignedTriangle,
    ray::Ray,
    sphere::{Sphere, SphereIntersection},
    triangle::{Triangle, TriangleIntersection},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Triangle(Triangle),
    AxiallyAlignedTriangle(AxiallyAlignedTriangle),
    Sphere(Sphere),
}

impl Shape {
    #[inline]
    pub fn min(&self) -> Vec3 {
        match self {
            Self::Triangle(g) => g.min(),
            Self::AxiallyAlignedTriangle(g) => g.min(),
            Self::Sphere(s) => s.min(),
        }
    }

    #[inline]
    pub fn max(&self) -> Vec3 {
        match self {
            Self::Triangle(g) => g.max(),
            Self::AxiallyAlignedTriangle(g) => g.max(),
            Self::Sphere(s) => s.max(),
        }
    }

    #[inline]
    pub fn intersect_ray(&self, ray: &Ray) -> Option<ShapeIntersection> {
        match self {
            Self::Triangle(g) => g.intersect_ray(ray).map(ShapeIntersection::Triangle),
            Self::AxiallyAlignedTriangle(g) => {
                g.intersect_ray(ray).map(ShapeIntersection::Triangle)
            }
            Self::Sphere(s) => s.intersect_ray(ray).map(ShapeIntersection::Sphere),
        }
    }

    #[inline]
    pub fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb> {
        match self {
            Self::Triangle(g) => g.clip_aabb(aabb),
            Self::AxiallyAlignedTriangle(g) => g.clip_aabb(aabb),
            Self::Sphere(_) => todo!(),
        }
    }
}

impl From<Triangle> for Shape {
    #[inline]
    fn from(value: Triangle) -> Self {
        value
            .as_axially_aligned()
            .map(Shape::AxiallyAlignedTriangle)
            .unwrap_or(Shape::Triangle(value))
    }
}

impl From<Sphere> for Shape {
    #[inline]
    fn from(value: Sphere) -> Self {
        Shape::Sphere(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShapeIntersection {
    Sphere(SphereIntersection),
    Triangle(TriangleIntersection),
}

impl ShapeIntersection {
    pub fn new_triangle(t: f32, u: f32, v: f32) -> Self {
        Self::Triangle(TriangleIntersection { t, u, v })
    }

    pub fn new_sphere(t: f32, normal: Vec3) -> Self {
        Self::Sphere(SphereIntersection { t, normal })
    }

    pub fn t(&self) -> f32 {
        match self {
            Self::Sphere(i) => i.t,
            Self::Triangle(i) => i.t,
        }
    }

    pub fn u(&self) -> Option<f32> {
        match self {
            Self::Sphere(_) => None,
            Self::Triangle(i) => Some(i.u),
        }
    }

    pub fn v(&self) -> Option<f32> {
        match self {
            Self::Sphere(_) => None,
            Self::Triangle(i) => Some(i.v),
        }
    }

    pub fn normal(&self) -> Option<Vec3> {
        match self {
            Self::Sphere(i) => Some(i.normal),
            Self::Triangle(_) => None,
        }
    }

    #[inline]
    pub fn point(&self, ray: &Ray) -> Vec3 {
        ray.param(self.t())
    }

    #[inline]
    pub fn ray(&self, ray: &Ray) -> Ray {
        ray.extended(self.t())
    }
}
