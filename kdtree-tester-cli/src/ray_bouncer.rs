use crate::checked_intersection::CheckedIntersection;
use geometry::{
    any_triangle::AnyTriangle,
    geometry::{IndexedIntersection, intersect_closest_geometry},
    ray::Ray,
    triangle::TriangleIntersection,
};
use glam::{UVec2, Vec2};
use kdtree::KdNode;
use rand::{SeedableRng, rngs::SmallRng};
use std::ops::RangeInclusive;
use tracing::{
    camera::Pinhole,
    light::Light,
    material::{Material, Surface},
    properties::TriangleProperties,
    sampling::uniform_sample_unit_square,
};

pub struct RayBouncer {
    pub geometries: Vec<AnyTriangle>,
    pub properties: Vec<TriangleProperties>,
    pub materials: Vec<Material>,
    pub lights: Vec<Light>,
    pub kdtree: KdNode,
    pub camera: Pinhole,
    pub bounces: u32,
    pub size: UVec2,
}

impl RayBouncer {
    fn reference_ray_intersect(
        &self,
        ray: &Ray,
        t_range: RangeInclusive<f32>,
    ) -> Option<IndexedIntersection<TriangleIntersection>> {
        let indices = 0u32..self.geometries.len() as u32;
        intersect_closest_geometry(&self.geometries, indices, ray, t_range)
    }

    fn checked_ray_intersect(
        &self,
        ray: &Ray,
        t_range: RangeInclusive<f32>,
    ) -> CheckedIntersection<TriangleIntersection> {
        let kdtree = self
            .kdtree
            .intersect(&self.geometries, ray, t_range.clone());
        let reference = self.reference_ray_intersect(ray, t_range);
        CheckedIntersection {
            ray: ray.clone(),
            reference,
            kdtree,
        }
    }

    fn bounce(
        &self,
        mut rng: SmallRng,
        ray: &Ray,
        accumulated_bounces: u32,
    ) -> Option<CheckedIntersection<TriangleIntersection>> {
        if accumulated_bounces >= self.bounces {
            return None;
        }

        let intersection = self.checked_ray_intersect(ray, 0.0..=f32::MAX);
        if !intersection.is_valid() {
            return Some(intersection);
        }
        let IndexedIntersection { index, inner } = intersection.reference?;
        let properties = &self.properties[index as usize];

        let wi = -ray.direction;
        let n = properties.compute_normal(&inner);
        let uv = properties.compute_texcoord(&inner);
        let material = &self.materials[properties.material];
        // TODO: How to chose offset?
        let offset = 0.00001 * n;
        let point = ray.param(inner.t);
        let point_above = point + offset;
        let point_below = point - offset;

        let incoming_fails = self
            .lights
            .iter()
            .filter_map(|light| {
                let (shadow_ray, t_range) = light.sample_shadow_ray(point_above, &mut rng);
                let shadow = self.checked_ray_intersect(&shadow_ray, t_range);
                (!shadow.is_valid()).then_some(shadow)
            })
            .collect::<Vec<_>>();
        if let Some(checked) = incoming_fails.first() {
            return Some(checked.clone());
        }

        let sample = material.sample(&Surface { wi, n, uv }, &mut rng);
        let next_ray = Ray::new(
            if sample.wo.dot(n) >= 0.0 {
                point_above
            } else {
                point_below
            },
            sample.wo,
        );

        self.bounce(rng, &next_ray, accumulated_bounces + 1)
    }

    pub fn bounce_pixel(
        &self,
        pixel: (u32, u32),
    ) -> Option<CheckedIntersection<TriangleIntersection>> {
        let (x, y) = pixel;
        let mut rng = SmallRng::seed_from_u64(u64::from(y * self.size.y + x));
        let pixel_center = Vec2::new(x as f32, y as f32) + uniform_sample_unit_square(&mut rng);
        let scene_direction = pixel_center / self.size.as_vec2();
        let ray = self.camera.ray(scene_direction);
        self.bounce(rng, &ray, 0)
    }
}
