use glam::Vec3;
use rand::rngs::SmallRng;
use wavefront::mtl::{self};

use crate::sampling::uniform_sample_unit_sphere;

#[derive(Clone, Debug)]
pub struct SphericalLight {
    pub center: Vec3,
    pub intensity: Vec3,
    pub radius: f32,
}

impl SphericalLight {
    pub fn new(center: Vec3, radius: f32, color: Vec3, intensity: f32) -> SphericalLight {
        SphericalLight {
            center,
            intensity: color * intensity,
            radius,
        }
    }

    pub fn emitted(&self, point: Vec3) -> Vec3 {
        self.intensity / (self.center - point).length_squared()
    }

    pub fn sample(&self, rng: &mut SmallRng) -> Vec3 {
        self.center + uniform_sample_unit_sphere(rng) * self.radius
    }
}

impl From<&mtl::Light> for SphericalLight {
    fn from(value: &mtl::Light) -> Self {
        Self::new(
            value.position.into(),
            value.radius,
            value.color.into(),
            value.intensity,
        )
    }
}
