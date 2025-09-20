use std::ops::RangeInclusive;

use geometry::{
    any_triangle::AnyTriangle,
    geometry::{IndexedIntersection, Intersection, intersect_closest_geometry},
    ray::Ray,
    sphere::{Sphere, SphereIntersection},
    triangle::TriangleIntersection,
};
use glam::{Vec2, Vec3};
use kdtree::KdNode;

use crate::{
    material::Material,
    properties::{SphereProperties, TriangleProperties},
};

pub trait GeometryCollection {
    type Intersection: Intersection;

    fn intersect(
        &self,
        ray: &Ray,
        t_range: RangeInclusive<f32>,
    ) -> Option<IndexedIntersection<Self::Intersection>>;

    fn compute_normal(&self, intersection: &IndexedIntersection<Self::Intersection>) -> Vec3;
    fn compute_texcoord(&self, intersection: &IndexedIntersection<Self::Intersection>) -> Vec2;
    fn material(&self, intersection: &IndexedIntersection<Self::Intersection>) -> &Material;
}

pub struct TriangleCollection {
    pub triangles: Vec<AnyTriangle>,
    pub properties: Vec<TriangleProperties>,
    pub materials: Vec<Material>,
    pub kdtree: KdNode,
}

impl GeometryCollection for TriangleCollection {
    type Intersection = TriangleIntersection;

    #[inline]
    fn intersect(
        &self,
        ray: &Ray,
        t_range: RangeInclusive<f32>,
    ) -> Option<IndexedIntersection<Self::Intersection>> {
        self.kdtree.intersect(&self.triangles, ray, t_range)
    }

    #[inline]
    fn compute_normal(&self, intersection: &IndexedIntersection<Self::Intersection>) -> Vec3 {
        self.properties[intersection.index as usize].compute_normal(&intersection.inner)
    }

    #[inline]
    fn compute_texcoord(&self, intersection: &IndexedIntersection<Self::Intersection>) -> Vec2 {
        self.properties[intersection.index as usize].compute_texcoord(&intersection.inner)
    }

    #[inline]
    fn material(&self, intersection: &IndexedIntersection<Self::Intersection>) -> &Material {
        &self.materials[self.properties[intersection.index as usize].material]
    }
}

pub struct SphereCollection {
    pub spheres: Vec<Sphere>,
    pub properties: Vec<SphereProperties>,
    pub materials: Vec<Material>,
}

impl GeometryCollection for SphereCollection {
    type Intersection = SphereIntersection;

    #[inline]
    fn intersect(
        &self,
        ray: &Ray,
        t_range: RangeInclusive<f32>,
    ) -> Option<IndexedIntersection<Self::Intersection>> {
        intersect_closest_geometry(
            &self.spheres,
            0u32..(self.spheres.len() as u32),
            ray,
            t_range,
        )
    }

    #[inline]
    fn compute_normal(&self, intersection: &IndexedIntersection<Self::Intersection>) -> Vec3 {
        self.properties[intersection.index as usize].compute_normal(&intersection.inner)
    }

    #[inline]
    fn compute_texcoord(&self, intersection: &IndexedIntersection<Self::Intersection>) -> Vec2 {
        self.properties[intersection.index as usize].compute_texcoord(&intersection.inner)
    }

    #[inline]
    fn material(&self, intersection: &IndexedIntersection<Self::Intersection>) -> &Material {
        &self.materials[self.properties[intersection.index as usize].material]
    }
}
