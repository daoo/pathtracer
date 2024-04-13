use crate::{
    camera::Pinhole, image_buffer::ImageBuffer, light::SphericalLight, material::Material,
    scene::Scene,
};
use geometry::ray::Ray;
use nalgebra::{UnitVector3, Vector2, Vector3};

fn light_contribution(
    scene: &Scene,
    material: &dyn Material,
    target: Vector3<f32>,
    offset_point: Vector3<f32>,
    wi: Vector3<f32>,
    n: UnitVector3<f32>,
    light: &SphericalLight,
) -> Vector3<f32> {
    let direction = light.center - target;
    let shadow_ray = Ray {
        origin: offset_point,
        direction,
    };
    if scene.intersect_any(&shadow_ray, 0.0..=1.0) {
        return Vector3::zeros();
    }

    let wr = direction.normalize();
    let radiance = light.emitted(target);
    material.brdf(&wi, &wr, &n).component_mul(&radiance) * wr.dot(&n).abs()
}

fn trace_ray(scene: &Scene, ray: &Ray) -> Vector3<f32> {
    let intersection = scene.intersect(ray, 0.0..=f32::MAX);
    if intersection.is_none() {
        return scene.environment;
    }
    let (triangle_index, intersection) = intersection.unwrap();
    let triangle = &scene.triangle_data[triangle_index as usize];

    let wi = -ray.direction;
    let point = ray.param(intersection.t);
    let n = triangle.normals.lerp(intersection.u, intersection.v);

    // TODO: How to chose offset?
    let offset_point = point + 0.0001 * n.into_inner();

    let material = triangle.material.as_ref();
    scene
        .lights
        .iter()
        .map(|light| light_contribution(scene, material, point, offset_point, wi, n, light))
        .sum()
}

pub fn render(scene: &Scene, camera: &Pinhole, buffer: &mut ImageBuffer) {
    let buffer_size = Vector2::new(buffer.width() as f32, buffer.height() as f32);
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            let pixel_center = Vector2::new(x as f32, y as f32) + Vector2::new(0.5, 0.5);
            let scene_direction = pixel_center.component_div(&buffer_size);
            let ray = camera.ray(scene_direction.x, scene_direction.y);
            let value = trace_ray(scene, &ray);
            buffer.add_pixel_mut(x, y, value.into());
        }
    }
}
