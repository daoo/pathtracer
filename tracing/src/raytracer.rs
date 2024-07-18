use std::ops::RangeInclusive;

use crate::{
    image_buffer::ImageBuffer,
    material::{Material, OutgoingRay},
};
use geometry::ray::Ray;
use glam::{UVec2, Vec2, Vec3};
use kdtree::{intersection::KdIntersection, KdTree};
use scene::{camera::Pinhole, material::MaterialModel, Scene};

pub struct Raytracer {
    pub scene: Scene,
    pub kdtree: KdTree,
    pub camera: Pinhole,
}

impl Raytracer {
    fn intersect(&self, ray: &Ray, t_range: RangeInclusive<f32>) -> Option<KdIntersection> {
        self.kdtree.intersect(ray, t_range)
    }

    fn intersect_any(&self, ray: &Ray, t_range: RangeInclusive<f32>) -> bool {
        self.intersect(ray, t_range).is_some()
    }

    fn trace_ray(&self, ray: &Ray) -> Vec3 {
        let intersection = self.intersect(ray, 0.0..=f32::MAX);
        if intersection.is_none() {
            return self.scene.environment;
        }
        let intersection = intersection.unwrap();
        let triangle = &self.scene.triangle_data[intersection.index as usize];
        let intersection = intersection.intersection;

        let wi = -ray.direction;
        let point = ray.param(intersection.t);
        let n = triangle.normals.lerp(intersection.u, intersection.v);
        let uv = triangle.texcoords.lerp(intersection.u, intersection.v);

        // TODO: How to chose offset?
        let offset_point = point + 0.0001 * n;

        self.scene
            .lights
            .iter()
            .map(|light| {
                let this = &self;
                let material: &MaterialModel = &triangle.material;
                let direction = light.center - point;
                let shadow_ray = Ray::new(offset_point, direction);
                if this.intersect_any(&shadow_ray, 0.0..=1.0) {
                    return Vec3::ZERO;
                }
                let wo = direction.normalize();
                let radiance = light.emitted(point);
                material.brdf(&OutgoingRay { wi, n, wo, uv }) * radiance * wo.dot(n).abs()
            })
            .sum()
    }

    pub fn render(&self, buffer: &mut ImageBuffer) {
        let buffer_size = buffer.size.as_vec2();
        for y in 0..buffer.size.y {
            for x in 0..buffer.size.x {
                let pixel = UVec2::new(x, y);
                let pixel_center = Vec2::new(pixel.x as f32 + 0.5, pixel.y as f32 + 0.5);
                let scene_direction = pixel_center / buffer_size;
                let ray = self.camera.ray(scene_direction);
                buffer[pixel] += self.trace_ray(&ray);
            }
        }
    }

    pub fn camera(&self) -> &Pinhole {
        &self.camera
    }
}
