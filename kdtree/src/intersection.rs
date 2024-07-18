use std::{cmp, ops::RangeInclusive};

use geometry::{geometry::Geometry, intersection::RayIntersection, ray::Ray};

#[derive(Debug, Clone, PartialEq)]
pub struct KdIntersection {
    pub index: u32,
    pub intersection: RayIntersection,
}

impl KdIntersection {
    #[inline]
    pub fn new(index: u32, intersection: RayIntersection) -> Self {
        KdIntersection {
            index,
            intersection,
        }
    }

    #[inline]
    pub fn cmp_by_ray_param(&self, other: &Self) -> cmp::Ordering {
        f32::total_cmp(&self.intersection.t, &other.intersection.t)
    }
}

pub fn intersect_closest_geometry(
    geometries: &[Geometry],
    indices: impl Iterator<Item = u32>,
    ray: &Ray,
    t_range: RangeInclusive<f32>,
) -> Option<KdIntersection> {
    indices
        .filter_map(|index| {
            let geometry = unsafe { geometries.get_unchecked(index as usize) };
            geometry.intersect_ray(ray).and_then(|intersection| {
                t_range.contains(&intersection.t).then_some(KdIntersection {
                    index,
                    intersection,
                })
            })
        })
        .min_by(KdIntersection::cmp_by_ray_param)
}
