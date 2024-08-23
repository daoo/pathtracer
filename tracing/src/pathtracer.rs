use crate::{
    image_buffer::ImageBuffer,
    material::{material_brdf, material_sample, IncomingRay, OutgoingRay},
    raylogger::{RayLoggerWithIteration, RayLoggerWithIterationAndPixel},
    sampling::{sample_light, uniform_sample_unit_square},
};
use geometry::ray::Ray;
use glam::{UVec2, Vec3};
use kdtree::KdNode;
use rand::rngs::SmallRng;
use scene::{camera::Pinhole, Scene};

pub struct Pathtracer {
    pub max_bounces: u8,
    pub scene: Scene,
    pub kdtree: KdNode,
}

impl Pathtracer {
    fn trace_ray(
        &self,
        mut ray_logger: RayLoggerWithIterationAndPixel,
        rng: &mut SmallRng,
        accumulated_radiance: Vec3,
        accumulated_transport: Vec3,
        accumulated_bounces: u8,
        ray: &Ray,
    ) -> Vec3 {
        if accumulated_bounces >= self.max_bounces {
            return accumulated_radiance;
        }

        let intersection = self
            .kdtree
            .intersect(&self.scene.geometries, ray, 0.0..=f32::MAX);
        if intersection.is_none() {
            ray_logger.log_infinite(ray, accumulated_bounces).unwrap();
            return accumulated_radiance + accumulated_transport * self.scene.environment;
        }
        let intersection = intersection.unwrap();
        let intersection_index = intersection.index;
        let intersection = intersection.intersection;

        ray_logger
            .log_finite(&ray.extended(intersection.t), accumulated_bounces)
            .unwrap();

        let wi = -ray.direction;
        let n = self.scene.get_normal(intersection_index, &intersection);
        let uv = self.scene.get_texcoord(intersection_index, &intersection);

        // TODO: How to chose offset?
        let offset = 0.00001 * n;
        let point = ray.param(intersection.t);
        let point_above = point + offset;
        let point_below = point - offset;

        let incoming_radiance: Vec3 = self
            .scene
            .lights
            .iter()
            .map(|light| {
                let shadow_ray = Ray::between(point_above, sample_light(light, rng));
                let intersection =
                    self.kdtree
                        .intersect(&self.scene.geometries, &shadow_ray, 0.0..=1.0);
                if intersection.is_some() {
                    return Vec3::ZERO;
                }
                let wo = shadow_ray.direction.normalize();
                let radiance = light.emitted(point);
                let brdf = material_brdf(
                    self.scene.get_material(intersection_index),
                    &OutgoingRay { wi, n, uv, wo },
                );
                brdf * radiance * wo.dot(n).abs()
            })
            .sum();

        let accumulated_radiance = accumulated_radiance + accumulated_transport * incoming_radiance;

        let sample = material_sample(
            self.scene.get_material(intersection_index),
            &IncomingRay { wi, n, uv },
            rng,
        );
        if sample.pdf <= 0.01 {
            return accumulated_radiance;
        }

        let cosine_term = sample.wo.dot(n);
        let accumulated_transport =
            accumulated_transport * sample.brdf * (cosine_term.abs() / sample.pdf);
        if accumulated_transport.length_squared() <= 1.0e-4 {
            return accumulated_radiance;
        }

        let next_start_point = if cosine_term >= 0.0 {
            point_above
        } else {
            point_below
        };
        let next_ray = Ray::new(next_start_point, sample.wo);

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
        ray_logger: RayLoggerWithIterationAndPixel,
        rng: &mut SmallRng,
        ray: &Ray,
    ) -> Vec3 {
        self.trace_ray(
            ray_logger,
            rng,
            Vec3::ZERO,
            Vec3::new(1.0, 1.0, 1.0),
            0,
            ray,
        )
    }

    fn sample_ray_for_pixel(pinhole: &Pinhole, rng: &mut SmallRng, pixel: UVec2) -> Ray {
        debug_assert!(pixel.x < pinhole.size.x && pixel.y < pinhole.size.y);
        let pixel_center = pixel.as_vec2() + uniform_sample_unit_square(rng);
        pinhole.ray(pixel_center / pinhole.size.as_vec2())
    }

    fn render_pixel(
        &self,
        pinhole: &Pinhole,
        pixel: UVec2,
        ray_logger: &mut RayLoggerWithIteration,
        rng: &mut SmallRng,
    ) -> Vec3 {
        let ray_logger = ray_logger.with_pixel(pixel.x as u16, pixel.y as u16);
        let ray = Self::sample_ray_for_pixel(pinhole, rng, pixel);
        self.trace_ray_defaults(ray_logger, rng, &ray)
    }

    pub fn render_mut(
        &self,
        pinhole: &Pinhole,
        ray_logger: &mut RayLoggerWithIteration,
        rng: &mut SmallRng,
        buffer: &mut ImageBuffer,
    ) {
        for y in 0..buffer.size.y {
            for x in 0..buffer.size.x {
                let pixel = UVec2::new(x, y);
                buffer[pixel] += self.render_pixel(pinhole, pixel, ray_logger, rng);
            }
        }
    }

    pub fn render_subdivided_mut(
        &self,
        pinhole: &Pinhole,
        ray_logger: &mut RayLoggerWithIteration,
        rng: &mut SmallRng,
        buffer: &mut ImageBuffer,
        sub_start: UVec2,
        sub_size: UVec2,
    ) {
        for sub_y in 0..sub_size.y {
            for sub_x in 0..sub_size.x {
                let pixel = sub_start + UVec2::new(sub_x, sub_y);
                buffer[pixel] += self.render_pixel(pinhole, pixel, ray_logger, rng);
            }
        }
    }
}
