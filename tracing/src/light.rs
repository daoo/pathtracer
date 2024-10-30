use std::ops::RangeInclusive;

use geometry::ray::Ray;
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
    fn emitted(&self, point: &Vec3) -> Vec3 {
        self.intensity / (self.center - point).length_squared()
    }

    fn sample(&self) -> (Vec3, RangeInclusive<f32>) {
        (self.center, 0.0..=1.0)
    }
}

#[derive(Clone, Debug)]
pub struct SphericalLight {
    pub point: PointLight,
    pub radius: f32,
}

impl SphericalLight {
    #[inline]
    fn sample(&self, rng: &mut SmallRng) -> (Vec3, RangeInclusive<f32>) {
        let target = self.point.center + uniform_sample_unit_sphere(rng) * self.radius;
        (target, 0.0..=1.0)
    }
}

#[derive(Clone, Debug)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub intensity: Vec3,
}

impl DirectionalLight {
    fn sample(&self, point: Vec3) -> (Vec3, RangeInclusive<f32>) {
        (point - self.direction, 0.0f32..=f32::MAX)
    }
}

#[derive(Clone, Debug)]
pub enum Light {
    PointLight(PointLight),
    SphericalLight(SphericalLight),
    DirectionalLight(DirectionalLight),
}

impl Light {
    #[inline]
    pub fn emitted(&self, point: &Vec3) -> Vec3 {
        match self {
            Light::PointLight(light) => light.emitted(point),
            Light::SphericalLight(light) => light.point.emitted(point),
            Light::DirectionalLight(light) => light.intensity,
        }
    }

    pub fn sample_shadow_ray(&self, point: Vec3, rng: &mut SmallRng) -> (Ray, RangeInclusive<f32>) {
        let (target, t_range) = match self {
            Light::PointLight(light) => light.sample(),
            Light::SphericalLight(light) => light.sample(rng),
            Light::DirectionalLight(light) => light.sample(point),
        };
        (Ray::between(point, target), t_range)
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

impl From<DirectionalLight> for Light {
    fn from(value: DirectionalLight) -> Self {
        Self::DirectionalLight(value)
    }
}
