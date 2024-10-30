use glam::Vec3;
use rand::rngs::SmallRng;
use wavefront::mtl::{self};

use crate::sampling::uniform_sample_unit_sphere;

#[derive(Clone, Debug)]
pub struct PointLight {
    pub center: Vec3,
    pub intensity: Vec3,
}

impl PointLight {
    pub fn emitted(&self, point: &Vec3) -> Vec3 {
        self.intensity / (self.center - point).length_squared()
    }
}

#[derive(Clone, Debug)]
pub struct SphericalLight {
    pub point: PointLight,
    pub radius: f32,
}

impl SphericalLight {
    pub fn sample(&self, rng: &mut SmallRng) -> Vec3 {
        self.point.center + uniform_sample_unit_sphere(rng) * self.radius
    }
}

#[derive(Clone, Debug)]
pub enum Light {
    PointLight(PointLight),
    SphericalLight(SphericalLight),
}

impl Light {
    pub fn emitted(&self, point: &Vec3) -> Vec3 {
        match self {
            Light::PointLight(light) => light.emitted(point),
            Light::SphericalLight(light) => light.point.emitted(point),
        }
    }

    pub fn sample(&self, rng: &mut SmallRng) -> Vec3 {
        match self {
            Light::PointLight(light) => light.center,
            Light::SphericalLight(light) => light.sample(rng),
        }
    }
}

impl From<&mtl::Light> for Light {
    fn from(value: &mtl::Light) -> Self {
        Self::SphericalLight(SphericalLight {
            point: PointLight {
                center: value.position.into(),
                intensity: Vec3::from(value.color) * value.intensity,
            },
            radius: value.radius,
        })
    }
}

impl From<PointLight> for Light {
    fn from(value: PointLight) -> Self {
        Self::PointLight(value)
    }
}

impl From<SphericalLight> for Light {
    fn from(value: SphericalLight) -> Self {
        Self::SphericalLight(value)
    }
}
