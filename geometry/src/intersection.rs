use std::ops::RangeInclusive;

use glam::Vec3;

use crate::{geometry::Geometry, ray::Ray};

#[derive(Debug, Clone, PartialEq)]
pub struct PointIntersection {
    pub(crate) u: f32,
    pub(crate) v: f32,
}

impl PointIntersection {
    #[inline]
    pub(crate) fn new(u: f32, v: f32) -> PointIntersection {
        PointIntersection { u, v }
    }

    #[inline]
    pub(crate) fn with_ray_param(&self, t: f32) -> RayIntersection {
        RayIntersection {
            t,
            u: self.u,
            v: self.v,
        }
    }
}

impl From<&RayIntersection> for PointIntersection {
    #[inline]
    fn from(value: &RayIntersection) -> Self {
        PointIntersection {
            u: value.u,
            v: value.v,
        }
    }
}

impl From<&GeometryIntersection> for PointIntersection {
    #[inline]
    fn from(value: &GeometryIntersection) -> Self {
        PointIntersection {
            u: value.inner.u,
            v: value.inner.v,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RayIntersection {
    pub t: f32,
    pub u: f32,
    pub v: f32,
}

impl RayIntersection {
    #[inline]
    pub fn new(t: f32, u: f32, v: f32) -> Self {
        RayIntersection { t, u, v }
    }

    #[inline]
    pub fn point(&self, ray: &Ray) -> Vec3 {
        ray.param(self.t)
    }

    #[inline]
    pub fn ray(&self, ray: &Ray) -> Ray {
        ray.extended(self.t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeometryIntersection {
    pub index: u32,
    pub inner: RayIntersection,
}

impl GeometryIntersection {
    #[inline]
    pub fn new(index: u32, inner: RayIntersection) -> Self {
        GeometryIntersection { index, inner }
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        if self.inner.t <= other.inner.t {
            self
        } else {
            other
        }
    }

    #[inline]
    pub fn point(&self, ray: &Ray) -> Vec3 {
        self.inner.point(ray)
    }

    #[inline]
    pub fn ray(&self, ray: &Ray) -> Ray {
        self.inner.ray(ray)
    }
}

pub fn intersect_closest_geometry(
    geometries: &[Geometry],
    indices: impl Iterator<Item = u32>,
    ray: &Ray,
    t_range: RangeInclusive<f32>,
) -> Option<GeometryIntersection> {
    indices
        .filter_map(|index| {
            let geometry = unsafe { geometries.get_unchecked(index as usize) };
            geometry.intersect_ray(ray).and_then(|inner| {
                t_range
                    .contains(&inner.t)
                    .then_some(GeometryIntersection { index, inner })
            })
        })
        .reduce(GeometryIntersection::min)
}
