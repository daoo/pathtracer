use crate::camera::*;
use crate::geometry::ray::*;
use crate::light::*;
use crate::material::*;
use crate::scene::*;
use nalgebra::{vector, Vector2, Vector3, DMatrix};

fn light_contribution<M>(
    scene: &Scene,
    material: &M,
    target: Vector3<f32>,
    offset_point: Vector3<f32>,
    wi: Vector3<f32>,
    n: Vector3<f32>,
    light: &SphericalLight) -> Vector3<f32>
where
M: Material,
{
    let direction = light.center - target;
    let shadow_ray = Ray { origin: offset_point, direction };
    if scene.intersect_any(&shadow_ray, 0.0, 1.0) {
        return Vector3::zeros()
    }

    let wr = direction.normalize();
    let radiance = light.emitted(target);
    material.brdf(&wi, &wr, &n).component_mul(&radiance) * wr.dot(&n).abs()
}

fn environment_contribution(_: &Ray) -> Vector3<f32> {
    vector![0.8, 0.8, 0.8]
}

fn trace_ray(scene: &Scene, ray: &Ray) -> Vector3<f32> {
    let intersection = scene.intersect(ray, 0.0, std::f32::MAX);
    if intersection.is_none() {
        return environment_contribution(ray);
    }
    let (triangle_index, intersection) = intersection.unwrap();

    let wi = -ray.direction;
    let point = ray.param(intersection.t);
    let n = vector![1.0, 0.0, 0.0]; // TODO

    // TODO: How to chose offset?
    let offset_point = point + 0.000001 * n;

    let material = DiffuseReflectiveMaterial { reflectance: vector![0.8, 0.8, 0.8] };

    scene.lights.iter()
        .map(|light| light_contribution(
                scene, &material, point, offset_point, wi, n, &light))
        .sum()
}

pub fn render(scene: &Scene, camera: &Pinhole, buffer: &mut DMatrix<Vector3<f32>>) {
    let buffer_size = vector![buffer.ncols() as f32, buffer.nrows() as f32];
    for y in 0..buffer.nrows() {
        for x in 0..buffer.ncols() {
            let pixel_center = Vector2::new(x as f32, y as f32) + Vector2::new(0.5, 0.5);
            let scene_direction = pixel_center.component_div(&buffer_size);
            let ray = camera.ray(scene_direction.x, scene_direction.y);
            buffer[(x, y)] += trace_ray(scene, &ray);
        }
    }
}
