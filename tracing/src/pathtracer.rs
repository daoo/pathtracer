use crate::{
    camera::Pinhole,
    collections::GeometryCollection,
    image_buffer::ImageBuffer,
    light::Light,
    material::Surface,
    raylogger::{RayLoggerWithIteration, RayLoggerWithIterationAndPixel},
    sampling::uniform_sample_unit_square,
};
use geometry::{geometry::Intersection, ray::Ray};
use glam::{UVec2, Vec3};
use rand::rngs::SmallRng;

pub struct Pathtracer<GC> {
    pub max_bounces: u8,
    pub geometry_collection: GC,
    pub lights: Vec<Light>,
    pub environment: Vec3,
}

impl<GC> Pathtracer<GC>
where
    GC: GeometryCollection,
{
    fn trace_ray(
        &self,
        mut ray_logger: RayLoggerWithIterationAndPixel,
        rng: &mut SmallRng,
        mut ray: Ray,
    ) -> Vec3 {
        let mut accumulated_radiance = Vec3::ZERO;
        let mut accumulated_transport = Vec3::ONE;
        for bounce in 1..=self.max_bounces {
            let intersection = self.geometry_collection.intersect(&ray, 0.0..=f32::MAX);
            ray_logger
                .log_ray(
                    &intersection
                        .as_ref()
                        .map_or(ray.clone(), |isect| ray.extended(isect.inner.t())),
                    bounce,
                    intersection.is_some(),
                )
                .unwrap();
            let Some(intersection) = intersection else {
                return accumulated_radiance + accumulated_transport * self.environment;
            };

            let wi = -ray.direction;
            let n = self.geometry_collection.compute_normal(&intersection);
            let uv = self.geometry_collection.compute_texcoord(&intersection);
            let material = self.geometry_collection.material(&intersection);

            // TODO: How to chose offset?
            // In PBRT the offset is chosen based on the surface normal, surface intersection
            // calculation error, sampled incoming direction and then rounded up to the next float.
            let offset = 0.00001 * n;
            let point = ray.param(intersection.inner.t());
            let point_above = point + offset;
            let point_below = point - offset;
            let surface = Surface { wi, n, uv };

            let sample = material.sample(&surface, rng);

            let incoming_radiance: Vec3 = self
                .lights
                .iter()
                .map(|light| {
                    // TODO: Offset should depend on incoming direction, not only surface normal.
                    let (shadow_ray, t_range) = light.sample_shadow_ray(point_above, rng);
                    let intersection = self.geometry_collection.intersect(&shadow_ray, t_range);
                    ray_logger
                        .log_shadow(&shadow_ray, bounce, intersection.is_some())
                        .unwrap();
                    if intersection.is_some() {
                        return Vec3::ZERO;
                    }
                    let radiance = light.emitted(&point);
                    let wo = shadow_ray.direction.normalize();
                    sample.brdf * radiance * wo.dot(surface.n).abs()
                })
                .sum();

            accumulated_radiance += accumulated_transport * incoming_radiance;

            if sample.pdf == 0.0 {
                return accumulated_radiance;
            }

            let cosine_term = sample.wo.dot(surface.n);
            if sample.is_delta {
                accumulated_transport *= sample.brdf;
            } else {
                accumulated_transport *= sample.brdf * (cosine_term.abs() / sample.pdf);
            }

            if accumulated_transport.length_squared() <= 1.0e-4 {
                return accumulated_radiance;
            }

            let next_start_point = if cosine_term >= 0.0 {
                point_above
            } else {
                point_below
            };
            ray = Ray::new(next_start_point, sample.wo);
        }
        accumulated_radiance
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
        self.trace_ray(ray_logger, rng, ray)
    }

    pub fn render_mut(
        &self,
        pinhole: &Pinhole,
        ray_logger: &mut RayLoggerWithIteration,
        rng: &mut SmallRng,
        buffer: &mut ImageBuffer,
    ) {
        for (pixel, value) in buffer.iter_mut() {
            *value += self.render_pixel(pinhole, pixel, ray_logger, rng);
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
