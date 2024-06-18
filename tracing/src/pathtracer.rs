use std::ops::RangeInclusive;

use crate::{
    image_buffer::ImageBuffer,
    material::{IncomingRay, Material, OutgoingRay},
    raylogger::{RayLoggerWithIteration, RayLoggerWithIterationAndPixel},
    sampling::{sample_light, uniform_sample_unit_square},
};
use geometry::{intersection::RayIntersection, ray::Ray};
use kdtree::KdTree;
use nalgebra::{Vector2, Vector3};
use rand::rngs::SmallRng;
use scene::{camera::Pinhole, Scene};

#[derive(Debug)]
pub struct Subdivision {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

impl Subdivision {
    fn width(&self) -> u32 {
        self.x2 - self.x1
    }

    fn height(&self) -> u32 {
        self.y2 - self.y1
    }

    fn pixels(&self) -> impl Iterator<Item = Vector2<u32>> + '_ {
        (self.y1..self.y2).flat_map(|y| (self.x1..self.y2).map(move |x| Vector2::new(x, y)))
    }
}

pub struct Pathtracer {
    pub max_bounces: u8,
    pub scene: Scene,
    pub kdtree: KdTree,
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
        ray_logger: &mut RayLoggerWithIterationAndPixel,
        rng: &mut SmallRng,
        accumulated_radiance: Vector3<f32>,
        accumulated_transport: Vector3<f32>,
        accumulated_bounces: u8,
        ray: &Ray,
    ) -> Vector3<f32> {
        if accumulated_bounces >= self.max_bounces {
            return accumulated_radiance;
        }

        let intersection = self.intersect(ray, 0.0..=f32::MAX);
        if intersection.is_none() {
            ray_logger.log_infinite(ray, accumulated_bounces).unwrap();
            return accumulated_radiance
                + accumulated_transport.component_mul(&self.scene.environment);
        }
        let (triangle_index, intersection) = intersection.unwrap();
        let triangle = &self.scene.triangle_data[triangle_index as usize];

        ray_logger
            .log_finite(&ray.extended(intersection.t), accumulated_bounces)
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
                let shadow_ray = Ray::between(&point_above, &sample_light(light, rng));
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

        let sample = material.sample(&IncomingRay { wi, n, uv }, rng);
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
            ray_logger,
            rng,
            accumulated_radiance,
            accumulated_transport,
            accumulated_bounces + 1,
            &next_ray,
        )
    }

    fn trace_ray_defaults(
        &self,
        ray_logger: &mut RayLoggerWithIterationAndPixel,
        rng: &mut SmallRng,
        ray: &Ray,
    ) -> Vector3<f32> {
        self.trace_ray(
            ray_logger,
            rng,
            Vector3::zeros(),
            Vector3::new(1.0, 1.0, 1.0),
            0,
            ray,
        )
    }

    fn sample_ray_for_pixel(
        &self,
        pinhole: &Pinhole,
        rng: &mut SmallRng,
        pixel: Vector2<u32>,
    ) -> Ray {
        debug_assert!(pixel.x < pinhole.width && pixel.y < pinhole.height);
        let pixel_center = pixel.cast() + uniform_sample_unit_square(rng);
        pinhole.ray(
            pixel_center.x / pinhole.width as f32,
            pixel_center.y / pinhole.height as f32,
        )
    }

    fn render_pixel(
        &self,
        pinhole: &Pinhole,
        pixel: Vector2<u32>,
        ray_logger: &mut RayLoggerWithIteration,
        rng: &mut SmallRng,
    ) -> Vector3<f32> {
        let mut ray_logger = ray_logger.with_pixel(pixel.x as u16, pixel.y as u16);
        let ray = self.sample_ray_for_pixel(pinhole, rng, pixel);
        self.trace_ray_defaults(&mut ray_logger, rng, &ray)
    }

    pub fn render(
        &self,
        pinhole: &Pinhole,
        subdivision: Subdivision,
        ray_logger: &mut RayLoggerWithIteration,
        rng: &mut SmallRng,
    ) -> ImageBuffer {
        let colors = subdivision
            .pixels()
            .flat_map(|pixel| -> [f32; 3] {
                self.render_pixel(pinhole, pixel, ray_logger, rng).into()
            })
            .collect::<Vec<_>>();
        ImageBuffer::from_vec(subdivision.width(), subdivision.height(), colors).unwrap()
    }

    pub fn render_mut(
        &self,
        pinhole: &Pinhole,
        ray_logger: &mut RayLoggerWithIteration,
        rng: &mut SmallRng,
        buffer: &mut ImageBuffer,
    ) {
        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let color = self.render_pixel(pinhole, Vector2::new(x, y), ray_logger, rng);
                buffer.add_pixel_mut(x, y, color.into());
            }
        }
    }
}
