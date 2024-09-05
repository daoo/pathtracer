use std::ops::RangeInclusive;

use crate::{geometry::Geometry, ray::Ray};

#[derive(Debug, Clone, PartialEq)]
pub struct PointIntersection {
    pub u: f32,
    pub v: f32,
}

impl PointIntersection {
    pub fn new(u: f32, v: f32) -> PointIntersection {
        PointIntersection { u, v }
    }

    pub fn with_ray_param(&self, t: f32) -> RayIntersection {
        RayIntersection {
            t,
            u: self.u,
            v: self.v,
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
    pub fn new(t: f32, u: f32, v: f32) -> Self {
        RayIntersection { t, u, v }
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
