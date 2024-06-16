use std::ops::{Range, RangeInclusive};

use crate::{
    image_buffer::ImageBuffer,
    material::{IncomingRay, Material, OutgoingRay},
    raylogger::RayLogger,
    sampling::{sample_light, uniform_sample_unit_square},
};
use geometry::{intersection::RayIntersection, ray::Ray};
use kdtree::KdTree;
use nalgebra::{Vector2, Vector3};
use rand::{rngs::SmallRng, SeedableRng};
use scene::{camera::Pinhole, Scene};

pub struct Pathtracer {
    pub max_bounces: u8,
    pub scene: Scene,
    pub kdtree: KdTree,
}

struct RayLoggerWithMeta<'a> {
    ray_logger: &'a mut RayLogger,
    iteration: u16,
    x: u16,
    y: u16,
}

impl<'a> RayLoggerWithMeta<'a> {
    fn log_infinite(&mut self, ray: &Ray, bounces: u8) -> Result<(), std::io::Error> {
        self.ray_logger
            .log_infinite(ray, self.iteration, (self.x, self.y), bounces)
    }

    fn log_finite(&mut self, ray: &Ray, bounces: u8) -> Result<(), std::io::Error> {
        self.ray_logger
            .log_finite(ray, self.iteration, (self.x, self.y), bounces)
    }
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
        ray_logger: &mut RayLoggerWithMeta,
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
        ray_logger: &mut RayLoggerWithMeta,
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

    fn ray(
        &self,
        camera: &Pinhole,
        buffer_size: &Vector2<f32>,
        rng: &mut SmallRng,
        x: u32,
        y: u32,
    ) -> Ray {
        let pixel_center = Vector2::new(x as f32, y as f32) + uniform_sample_unit_square(rng);
        let scene_direction = pixel_center.component_div(buffer_size);
        camera.ray(scene_direction.x, scene_direction.y)
    }

    pub fn render(
        &self,
        pinhole: &Pinhole,
        range_x: Range<u32>,
        range_y: Range<u32>,
    ) -> ImageBuffer {
        let width = range_x.len() as u32;
        let height = range_y.len() as u32;
        let buffer_size = Vector2::new(width as f32, height as f32);
        let mut rng = SmallRng::from_entropy();
        let vec = range_y
            .flat_map(|y| range_x.clone().map(move |x| (x, y)))
            .flat_map(|p| {
                let ray = self.ray(pinhole, &buffer_size, &mut rng, p.0, p.1);
                let mut ray_logger = RayLoggerWithMeta {
                    ray_logger: &mut RayLogger::None(),
                    iteration: 0,
                    x: p.0 as u16,
                    y: p.1 as u16,
                };
                let value = self.trace_ray_defaults(&mut ray_logger, &mut rng, &ray);
                Into::<[f32; 3]>::into(value)
            })
            .collect::<Vec<_>>();
        ImageBuffer::from_vec(width, height, vec).unwrap()
    }

    pub fn render_mut(
        &self,
        iteration: u16,
        camera: &Pinhole,
        ray_logger: &mut RayLogger,
        buffer: &mut ImageBuffer,
        rng: &mut SmallRng,
    ) {
        let buffer_size = Vector2::new(buffer.width() as f32, buffer.height() as f32);
        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let ray = self.ray(camera, &buffer_size, rng, x, y);
                let mut ray_logger = RayLoggerWithMeta {
                    ray_logger,
                    iteration,
                    x: x as u16,
                    y: y as u16,
                };
                let value = self.trace_ray_defaults(&mut ray_logger, rng, &ray);
                buffer.add_pixel_mut(x, y, value.into());
            }
        }
    }
}
