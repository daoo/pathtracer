use crate::camera::*;
use crate::geometry::ray::*;
use crate::image_buffer::ImageBuffer;
use crate::sampling::*;
use crate::scene::*;
use nalgebra::{vector, Vector2, Vector3};
use rand::rngs::SmallRng;

fn environment_contribution(_: &Ray) -> Vector3<f32> {
    vector![0.8, 0.8, 0.8]
}

fn trace_ray(
    iteration: u32,
    pixel: (u32, u32),
    max_bounces: u32,
    scene: &Scene,
    ray: Ray,
    accumulated_radiance: Vector3<f32>,
    accumulated_transport: Vector3<f32>,
    accumulated_bounces: u32,
    rng: &mut SmallRng,
) -> Vector3<f32> {
    if accumulated_bounces >= max_bounces {
        return accumulated_radiance;
    }

    let intersection = scene.intersect(&ray, 0.0, std::f32::MAX);
    if intersection.is_none() {
        if (365..375).contains(&pixel.0) && (425..435).contains(&pixel.1) {
            // eprintln!(
            //     "{},{},{},{},{},{},{},{},{}",
            //     iteration,
            //     pixel.0,
            //     pixel.1,
            //     ray.origin.x,
            //     ray.origin.y,
            //     ray.origin.z,
            //     ray.direction.x * 10.0,
            //     ray.direction.y * 10.0,
            //     ray.direction.z * 10.0
            // );
        }
        return accumulated_radiance
            + accumulated_transport.component_mul(&environment_contribution(&ray));
    }
    let (triangle_index, intersection) = intersection.unwrap();

    let wi = -ray.direction;
    let n = scene.triangle_normals[triangle_index].lerp(intersection.u, intersection.v);
    let material = &scene.triangle_materials[triangle_index];

    // TODO: How to chose offset?
    let offset = 0.00001 * n.into_inner();
    let point = ray.param(intersection.t);
    let point_above = point + offset;
    let point_below = point - offset;

    if (365..375).contains(&pixel.0) && (425..435).contains(&pixel.1) {
        // eprintln!(
        //     "{},{},{},{},{},{},{},{},{}",
        //     iteration,
        //     pixel.0,
        //     pixel.1,
        //     ray.origin.x,
        //     ray.origin.y,
        //     ray.origin.z,
        //     point.x,
        //     point.y,
        //     point.z
        // );
    }

    let incoming_radiance: Vector3<f32> = scene
        .lights
        .iter()
        .map(|light| {
            let shadow_ray = Ray::between(&point_above, &light.sample(rng));
            if scene.intersect_any(&shadow_ray, 0.0, 1.0) {
                return Vector3::zeros();
            }

            let wo = shadow_ray.direction.normalize();
            let radiance = light.emitted(point);

            material.brdf(&wi, &wo, &n).component_mul(&radiance) * wo.dot(&n).abs()
        })
        .sum();

    let accumulated_radiance =
        accumulated_radiance + accumulated_transport.component_mul(&incoming_radiance);

    let sample = material.sample(&wi, &n, rng);
    if sample.pdf < 0.00001 {
        return accumulated_radiance;
    }

    let cosine_term = sample.wo.dot(&n).abs();
    let accumulated_transport =
        accumulated_transport.component_mul(&(sample.brdf * (cosine_term / sample.pdf)));

    if accumulated_transport.norm_squared() <= 0.00001 * 0.00001 {
        return accumulated_radiance;
    }

    let next_ray = Ray {
        origin: if sample.wo.dot(&n) >= 0.0 {
            point_above
        } else {
            point_below
        },
        direction: *sample.wo,
    };
    trace_ray(
        iteration,
        pixel,
        max_bounces,
        scene,
        next_ray,
        accumulated_radiance,
        accumulated_transport,
        accumulated_bounces + 1,
        rng,
    )
}

pub fn render(
    iteration: u32,
    max_bounces: u32,
    scene: &Scene,
    camera: &Pinhole,
    buffer: &mut ImageBuffer,
    rng: &mut SmallRng,
) {
    let buffer_size = vector![buffer.ncols() as f32, buffer.nrows() as f32];
    for y in 0..buffer.nrows() {
        for x in 0..buffer.ncols() {
            let pixel_center = Vector2::new(x as f32, y as f32) + uniform_sample_unit_square(rng);
            let scene_direction = pixel_center.component_div(&buffer_size);
            let ray = camera.ray(scene_direction.x, scene_direction.y);
            buffer[(x, y)] += trace_ray(
                iteration,
                (x as u32, y as u32),
                max_bounces,
                scene,
                ray,
                Vector3::zeros(),
                Vector3::new(1.0, 1.0, 1.0),
                0,
                rng,
            );
        }
    }
}
