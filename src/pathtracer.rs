use crate::{
    camera::Pinhole, image_buffer::ImageBuffer, raylogger::RayLogger,
    sampling::uniform_sample_unit_square, scene::Scene,
};
use geometry::ray::Ray;
use nalgebra::{Vector2, Vector3};
use rand::rngs::SmallRng;

struct Pathtracer<'a> {
    iteration: u16,
    x: u16,
    y: u16,
    max_bounces: u16,
    scene: &'a Scene,
    rng: &'a mut SmallRng,
    ray_logger: &'a mut RayLogger,
}

impl<'a> Pathtracer<'a> {
    fn trace(
        &'a mut self,
        ray: &Ray,
        accumulated_radiance: Vector3<f32>,
        accumulated_transport: Vector3<f32>,
        accumulated_bounces: u16,
    ) -> Vector3<f32> {
        if accumulated_bounces >= self.max_bounces {
            return accumulated_radiance;
        }

        let intersection = self.scene.intersect(ray, 0.0..=std::f32::MAX);
        if intersection.is_none() {
            self.ray_logger
                .log_infinite(ray, self.iteration, (self.x, self.y))
                .unwrap();
            return accumulated_radiance
                + accumulated_transport.component_mul(&self.scene.environment);
        }
        let (triangle_index, intersection) = intersection.unwrap();
        let triangle = &self.scene.triangle_data[triangle_index as usize];

        let wi = -ray.direction;
        let n = triangle.normals.lerp(intersection.u, intersection.v);
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
                let shadow_ray = Ray::between(&point_above, &light.sample(self.rng));
                if self.scene.intersect_any(&shadow_ray, 0.0..=1.0) {
                    return Vector3::zeros();
                }

                let wo = shadow_ray.direction.normalize();
                let radiance = light.emitted(point);

                material.brdf(&wi, &wo, &n).component_mul(&radiance) * wo.dot(&n).abs()
            })
            .sum();

        let accumulated_radiance =
            accumulated_radiance + accumulated_transport.component_mul(&incoming_radiance);

        let sample = material.sample(&wi, &n, self.rng);
        let next_ray = Ray {
            origin: if sample.wo.dot(&n) >= 0.0 {
                point_above
            } else {
                point_below
            },
            direction: *sample.wo,
        };
        self.ray_logger
            .log_finite(
                &Ray {
                    origin: ray.origin,
                    direction: next_ray.origin,
                },
                self.iteration,
                (self.x, self.y),
            )
            .unwrap();

        if sample.pdf <= 0.01 {
            return accumulated_radiance;
        }

        let cosine_term = sample.wo.dot(&n).abs();
        let accumulated_transport =
            accumulated_transport.component_mul(&(sample.brdf * (cosine_term / sample.pdf)));

        if accumulated_transport.norm() <= 0.01 {
            return accumulated_radiance;
        }

        self.trace(
            &next_ray,
            accumulated_radiance,
            accumulated_transport,
            accumulated_bounces + 1,
        )
    }
}

pub fn render(
    iteration: u16,
    ray_logger: &mut RayLogger,
    max_bounces: u16,
    scene: &Scene,
    camera: &Pinhole,
    buffer: &mut ImageBuffer,
    rng: &mut SmallRng,
) {
    let buffer_size = Vector2::new(buffer.width() as f32, buffer.height() as f32);
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            let pixel_center = Vector2::new(x as f32, y as f32) + uniform_sample_unit_square(rng);
            let scene_direction = pixel_center.component_div(&buffer_size);
            let ray = camera.ray(scene_direction.x, scene_direction.y);
            let mut pathtracer = Pathtracer {
                iteration,
                x: x as u16,
                y: y as u16,
                max_bounces,
                scene,
                rng,
                ray_logger,
            };
            let value = pathtracer.trace(&ray, Vector3::zeros(), Vector3::new(1.0, 1.0, 1.0), 0);
            buffer.add_pixel_mut(x, y, value.into());
        }
    }
}
