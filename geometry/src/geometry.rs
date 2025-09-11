use std::ops::RangeInclusive;

use glam::Vec3;

use crate::{
    aabb::Aabb,
    any_triangle::AnyTriangle,
    axial_triangle::AxiallyAlignedTriangle,
    clip::clip_triangle_aabb,
    ray::Ray,
    sphere::{Sphere, SphereIntersection},
    triangle::{Triangle, TriangleIntersection},
};

pub trait Intersection {
    fn t(&self) -> f32;
}

impl Intersection for TriangleIntersection {
    fn t(&self) -> f32 {
        self.t
    }
}

impl Intersection for SphereIntersection {
    fn t(&self) -> f32 {
        self.t
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndexedIntersection<I> {
    pub index: u32,
    pub inner: I,
}

impl<I> IndexedIntersection<I>
where
    I: Intersection,
{
    #[inline]
    pub const fn new(index: u32, inner: I) -> Self {
        Self { index, inner }
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        if self.inner.t() <= other.inner.t() {
            self
        } else {
            other
        }
    }
}

pub trait Geometry {
    type Intersection: Intersection;

    fn min(&self) -> Vec3;
    fn max(&self) -> Vec3;

    fn intersect_ray(&self, ray: &Ray) -> Option<Self::Intersection>;

    fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb>;
}

impl Geometry for Triangle {
    type Intersection = TriangleIntersection;

    #[inline]
    fn min(&self) -> Vec3 {
        self.min()
    }

    #[inline]
    fn max(&self) -> Vec3 {
        self.max()
    }

    #[inline]
    fn intersect_ray(&self, ray: &Ray) -> Option<Self::Intersection> {
        self.intersect_ray(ray)
    }

    #[inline]
    fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb> {
        clip_triangle_aabb(&self.v0, &self.v1, &self.v2, aabb)
    }
}

impl Geometry for AxiallyAlignedTriangle {
    type Intersection = TriangleIntersection;

    #[inline]
    fn min(&self) -> Vec3 {
        self.min()
    }

    #[inline]
    fn max(&self) -> Vec3 {
        self.max()
    }

    #[inline]
    fn intersect_ray(&self, ray: &Ray) -> Option<Self::Intersection> {
        self.intersect_ray(ray)
    }

    #[inline]
    fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb> {
        clip_triangle_aabb(
            &self.plane.add_to(self.v0),
            &self.plane.add_to(self.v1),
            &self.plane.add_to(self.v2),
            aabb,
        )
    }
}

impl Geometry for Sphere {
    type Intersection = SphereIntersection;

    #[inline]
    fn min(&self) -> Vec3 {
        self.min()
    }

    #[inline]
    fn max(&self) -> Vec3 {
        self.max()
    }

    #[inline]
    fn intersect_ray(&self, ray: &Ray) -> Option<Self::Intersection> {
        self.intersect_ray(ray)
    }

    #[inline]
    fn clip_aabb(&self, _aabb: &Aabb) -> Option<Aabb> {
        todo!()
    }
}

impl Geometry for AnyTriangle {
    type Intersection = TriangleIntersection;

    fn min(&self) -> glam::Vec3 {
        match self {
            AnyTriangle::Triangle(t) => t.min(),
            AnyTriangle::AxiallyAlignedTriangle(t) => t.min(),
        }
    }

    fn max(&self) -> glam::Vec3 {
        match self {
            AnyTriangle::Triangle(t) => t.max(),
            AnyTriangle::AxiallyAlignedTriangle(t) => t.max(),
        }
    }

    fn intersect_ray(&self, ray: &Ray) -> Option<Self::Intersection> {
        match self {
            AnyTriangle::Triangle(t) => t.intersect_ray(ray),
            AnyTriangle::AxiallyAlignedTriangle(t) => t.intersect_ray(ray),
        }
    }

    fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb> {
        match self {
            AnyTriangle::Triangle(t) => t.clip_aabb(aabb),
            AnyTriangle::AxiallyAlignedTriangle(t) => t.clip_aabb(aabb),
        }
    }
}

pub fn intersect_closest_geometry<G>(
    geometries: &[G],
    indices: impl Iterator<Item = u32>,
    ray: &Ray,
    t_range: RangeInclusive<f32>,
) -> Option<IndexedIntersection<G::Intersection>>
where
    G: Geometry,
    G::Intersection: Intersection,
{
    indices
        .filter_map(|index| {
            let geometry = unsafe { geometries.get_unchecked(index as usize) };
            geometry.intersect_ray(ray).and_then(|intersection| {
                t_range
                    .contains(&intersection.t())
                    .then_some(IndexedIntersection::new(index, intersection))
            })
        })
        .reduce(IndexedIntersection::min)
}
