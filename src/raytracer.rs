use std::ops::RangeInclusive;

use crate::{
    camera::Pinhole, image_buffer::ImageBuffer, light::SphericalLight, material::Material,
    scene::Scene,
};
use geometry::{intersection::RayIntersection, ray::Ray};
use kdtree::KdTree;
use nalgebra::{UnitVector3, Vector2, Vector3};

pub struct Raytracer {
    pub scene: Scene,
    pub kdtree: KdTree,
    pub camera: Pinhole,
}

impl Raytracer {
    fn intersect(&self, ray: &Ray, t_range: RangeInclusive<f32>) -> Option<(u32, RayIntersection)> {
        self.kdtree.intersect(ray, t_range)
    }

    fn intersect_any(&self, ray: &Ray, t_range: RangeInclusive<f32>) -> bool {
        self.intersect(ray, t_range).is_some()
    }

    fn light_contribution(
        &self,
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
        if self.intersect_any(&shadow_ray, 0.0..=1.0) {
            return Vector3::zeros();
        }

        let wr = direction.normalize();
        let radiance = light.emitted(target);
        material.brdf(&wi, &wr, &n).component_mul(&radiance) * wr.dot(&n).abs()
    }

    fn trace_ray(&self, ray: &Ray) -> Vector3<f32> {
        let intersection = self.intersect(ray, 0.0..=f32::MAX);
        if intersection.is_none() {
            return self.scene.environment;
        }
        let (triangle_index, intersection) = intersection.unwrap();
        let triangle = &self.scene.triangle_data[triangle_index as usize];

        let wi = -ray.direction;
        let point = ray.param(intersection.t);
        let n = triangle.normals.lerp(intersection.u, intersection.v);

        // TODO: How to chose offset?
        let offset_point = point + 0.0001 * n.into_inner();

        let material = triangle.material.as_ref();
        self.scene
            .lights
            .iter()
            .map(|light| self.light_contribution(material, point, offset_point, wi, n, light))
            .sum()
    }

    pub fn render(&self, buffer: &mut ImageBuffer) {
        let buffer_size = Vector2::new(buffer.width() as f32, buffer.height() as f32);
        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let pixel_center = Vector2::new(x as f32, y as f32) + Vector2::new(0.5, 0.5);
                let scene_direction = pixel_center.component_div(&buffer_size);
                let ray = self.camera.ray(scene_direction.x, scene_direction.y);
                let value = self.trace_ray(&ray);
                buffer.add_pixel_mut(x, y, value.into());
            }
        }
    }
}
