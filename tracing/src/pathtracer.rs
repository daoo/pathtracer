use std::ops::RangeInclusive;

use crate::{
    image_buffer::ImageBuffer,
    material::{IncomingRay, Material, OutgoingRay},
    raylogger::RayLogger,
    sampling::{sample_light, uniform_sample_unit_square},
};
use geometry::{intersection::RayIntersection, ray::Ray};
use kdtree::KdTree;
use nalgebra::{Vector2, Vector3};
use rand::rngs::SmallRng;
use scene::{camera::Pinhole, Scene};

pub struct Pathtracer {
    pub max_bounces: u8,
    pub scene: Scene,
    pub kdtree: KdTree,
    pub camera: Pinhole,
}

struct Pixel<'a> {
    iteration: u16,
    x: u16,
    y: u16,
    rng: &'a mut SmallRng,
    ray_logger: &'a mut RayLogger,
}

impl Pathtracer {
    fn intersect(&self, ray: &Ray, t_range: RangeInclusive<f32>) -> Option<(u32, RayIntersection)> {
        self.kdtree.intersect(ray, t_range)
    }

    fn intersect_any(&self, ray: &Ray, t_range: RangeInclusive<f32>) -> bool {
        self.intersect(ray, t_range).is_some()
    }

    fn trace_ray(
        &self,
        pixel: &mut Pixel,
        ray: &Ray,
        accumulated_radiance: Vector3<f32>,
        accumulated_transport: Vector3<f32>,
        accumulated_bounces: u8,
    ) -> Vector3<f32> {
        if accumulated_bounces >= self.max_bounces {
            return accumulated_radiance;
        }

        let intersection = self.intersect(ray, 0.0..=std::f32::MAX);
        if intersection.is_none() {
            pixel
                .ray_logger
                .log_infinite(
                    ray,
                    pixel.iteration,
                    (pixel.x, pixel.y),
                    accumulated_bounces,
                )
                .unwrap();
            return accumulated_radiance
                + accumulated_transport.component_mul(&self.scene.environment);
        }
        let (triangle_index, intersection) = intersection.unwrap();
        let triangle = &self.scene.triangle_data[triangle_index as usize];

        pixel
            .ray_logger
            .log_finite(
                &ray.extended(intersection.t),
                pixel.iteration,
                (pixel.x, pixel.y),
                accumulated_bounces,
            )
            .unwrap();

        let wi = -ray.direction;
        let n = triangle.normals.lerp(intersection.u, intersection.v);
        let uv = triangle.texcoords.lerp(intersection.u, intersection.v);
        let material = triangle.material.as_ref();

        // TODO: How to chose offset?
        let offset = 0.00001 * n.into_inner();
        let point = ray.param(intersection.t);
        let point_above = point + offset;
        let point_below = point - offset;

        let incoming_radiance: Vector3<f32> = self
            .scene
            .lights
            .iter()
            .map(|light| {
                let shadow_ray = Ray::between(&point_above, &sample_light(light, pixel.rng));
                if self.intersect_any(&shadow_ray, 0.0..=1.0) {
                    return Vector3::zeros();
                }
                let wo = shadow_ray.direction.normalize();
                let radiance = light.emitted(point);
                material
                    .brdf(&OutgoingRay { wi, n, wo, uv })
                    .component_mul(&radiance)
                    * wo.dot(&n).abs()
            })
            .sum();

        let accumulated_radiance =
            accumulated_radiance + accumulated_transport.component_mul(&incoming_radiance);

        let sample = material.sample(&IncomingRay { wi, n, uv }, pixel.rng);
        let next_ray = Ray {
            origin: if sample.wo.dot(&n) >= 0.0 {
                point_above
            } else {
                point_below
            },
            direction: *sample.wo,
        };

        if sample.pdf <= 0.01 {
            return accumulated_radiance;
        }

        let cosine_term = sample.wo.dot(&n).abs();
        let accumulated_transport =
            accumulated_transport.component_mul(&(sample.brdf * (cosine_term / sample.pdf)));

        if accumulated_transport.norm() <= 0.01 {
            return accumulated_radiance;
        }

        self.trace_ray(
            pixel,
            &next_ray,
            accumulated_radiance,
            accumulated_transport,
            accumulated_bounces + 1,
        )
    }

    pub fn render(
        &self,
        iteration: u16,
        ray_logger: &mut RayLogger,
        buffer: &mut ImageBuffer,
        rng: &mut SmallRng,
    ) {
        let buffer_size = Vector2::new(buffer.width() as f32, buffer.height() as f32);
        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let pixel_center =
                    Vector2::new(x as f32, y as f32) + uniform_sample_unit_square(rng);
                let scene_direction = pixel_center.component_div(&buffer_size);
                let ray = self.camera.ray(scene_direction.x, scene_direction.y);
                let mut pixel = Pixel {
                    iteration,
                    x: x as u16,
                    y: y as u16,
                    rng,
                    ray_logger,
                };
                let value = self.trace_ray(
                    &mut pixel,
                    &ray,
                    Vector3::zeros(),
                    Vector3::new(1.0, 1.0, 1.0),
                    0,
                );
                buffer.add_pixel_mut(x, y, value.into());
            }
        }
    }
}
