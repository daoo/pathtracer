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
        mut ray: Ray,
    ) -> Vec3 {
        let mut accumulated_radiance = Vec3::ZERO;
        let mut accumulated_transport = Vec3::ONE;
        for bounce in 1..=self.max_bounces {
            let intersection = self
                .kdtree
                .intersect(self.scene.geometries(), &ray, 0.0..=f32::MAX);
            ray_logger
                .log_ray(
                    &intersection
                        .as_ref()
                        .map_or(ray.clone(), |isect| ray.extended(isect.inner.t)),
                    bounce,
                    intersection.is_some(),
                )
                .unwrap();
            if intersection.is_none() {
                return accumulated_radiance + accumulated_transport * self.scene.environment();
            }
            let intersection = self.scene.lookup_intersection(intersection.unwrap());

            let wi = -ray.direction;
            let n = intersection.normal;
            let uv = intersection.texcoord;
            let material = intersection.material;

            // TODO: How to chose offset?
            // In PBRT the offset is chosen based on the surface normal, surface intersection
            // calculation error, sampled incoming direction and then rounded up to the next float.
            let offset = 0.00001 * n;
            let point = ray.param(intersection.inner.inner.t);
            let point_above = point + offset;
            let point_below = point - offset;

            let incoming_radiance: Vec3 = self
                .scene
                .lights()
                .iter()
                .map(|light| {
                    // TODO: Offset should depend on incoming direction, not only surface normal.
                    let shadow_ray = Ray::between(point_above, sample_light(light, rng));
                    let intersection =
                        self.kdtree
                            .intersect(self.scene.geometries(), &shadow_ray, 0.0..=1.0);
                    ray_logger
                        .log_shadow(&shadow_ray, bounce, intersection.is_some())
                        .unwrap();
                    if intersection.is_some() {
                        return Vec3::ZERO;
                    }
                    let wo = shadow_ray.direction.normalize();
                    let radiance = light.emitted(point);
                    let brdf = material_brdf(material, &OutgoingRay { wi, n, uv, wo });
                    brdf * radiance * wo.dot(n).abs()
                })
                .sum();

            accumulated_radiance += accumulated_transport * incoming_radiance;

            let sample = material_sample(material, &IncomingRay { wi, n, uv }, rng);
            if sample.pdf <= 0.01 {
                return accumulated_radiance;
            }

            let cosine_term = sample.wo.dot(n);
            accumulated_transport *= sample.brdf * (cosine_term.abs() / sample.pdf);
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
