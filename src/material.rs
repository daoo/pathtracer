use crate::sampling::*;
use nalgebra::{RealField, vector, Vector3};
use rand::Rng;
use rand::rngs::SmallRng;

fn perpendicular(v: &Vector3<f32>) -> Vector3<f32> {
    if v.x.abs() < v.y.abs() {
        vector![0., -v.z, v.y]
    } else {
        vector![-v.z, 0., v.x]
    }

}

fn is_same_sign(a: f32, b: f32) -> bool {
    a.signum() == b.signum()
}

fn is_same_hemisphere(wi: &Vector3<f32>, wo: &Vector3<f32>, n: &Vector3<f32>) -> bool {
  is_same_sign(wi.dot(n), wo.dot(n))
}

#[derive(Debug)]
pub struct MaterialSample {
    pub pdf: f32,
    pub brdf: Vector3<f32>,
    pub wo: Vector3<f32>,
}

pub trait Material {
    fn brdf(&self, wo: &Vector3<f32>, wi: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32>;

    fn sample(&self, wi: &Vector3<f32>, n: &Vector3<f32>, rng: &mut SmallRng) -> MaterialSample;
}

struct DiffuseReflectiveMaterial {
    reflectance: Vector3<f32>,
}

impl Material for DiffuseReflectiveMaterial {
    fn brdf(&self, _: &Vector3<f32>, _: &Vector3<f32>, _: &Vector3<f32>) -> Vector3<f32> {
        self.reflectance * f32::frac_1_pi()
    }

    fn sample(&self, wi: &Vector3<f32>, n: &Vector3<f32>, rng: &mut SmallRng) -> MaterialSample {
        let tangent = perpendicular(n).normalize();
        let bitangent = n.cross(&tangent);
        let s = cosine_sample_hemisphere(rng);

        let wo = (s.x * tangent + s.y * bitangent + s.z * n).normalize();
        return MaterialSample{
            pdf: s.norm(),
            brdf: self.brdf(&wi, &wo, &n),
            wo
        }
    }
}

struct SpecularReflectiveMaterial {
    reflectance: Vector3<f32>,
}

impl Material for SpecularReflectiveMaterial {
    fn brdf(&self, _: &Vector3<f32>, _: &Vector3<f32>, _: &Vector3<f32>) -> Vector3<f32> {
        Vector3::zeros()
    }

    fn sample(&self, wi: &Vector3<f32>, n: &Vector3<f32>, _: &mut SmallRng) -> MaterialSample {
        let wo = (2.0 * wi.dot(n).abs() * n - wi).normalize();
        let pdf = if is_same_hemisphere(&wi, &wo, &n) { wo.dot(n).abs() } else { 0.0 };
        return MaterialSample { pdf, brdf: self.reflectance, wo };
    }
}

struct SpecularRefractiveMaterial {
  reflection: SpecularReflectiveMaterial,
  index_of_refraction: f32,
}

impl Material for SpecularRefractiveMaterial {
    fn brdf(&self, _: &Vector3<f32>, _: &Vector3<f32>, _: &Vector3<f32>) -> Vector3<f32> {
        Vector3::zeros()
    }

    fn sample(&self, wi: &Vector3<f32>, n: &Vector3<f32>, rng: &mut SmallRng) -> MaterialSample {
        let a = (-wi).dot(n);
        let eta = if a < 0.0  { 1.0 / self.index_of_refraction } else { self.index_of_refraction };
        let n = if a < 0.0 { *n } else { -n };

        let w = -a * eta;
        let k = 1.0 + (w - eta) * (w + eta);
        if k < 0.0 {
            // Total internal reflection
            return self.reflection.sample(&wi, &n, rng);
        }

        let k = k.sqrt();
        let wo = (-eta * wi + (w - k) * n).normalize();
        MaterialSample { pdf: 1.0, brdf: vector![1., 1., 1.], wo }
    }
}

struct FresnelBlendMaterial<MReflective, MRefractive> {
    reflection: MReflective,
    refraction: MRefractive,
    r0: f32,
}

fn reflectance(r0: f32, wo: &Vector3<f32>, n: &Vector3<f32>) -> f32 {
  r0 + (1.0 - r0) * (1.0 - wo.dot(n).abs().powf(5.0))
}

impl<MReflective, MRefractive> Material for FresnelBlendMaterial<MReflective, MRefractive>
    where
        MReflective: Material,
        MRefractive: Material,
{
    fn brdf(&self, wo: &Vector3<f32>, wi: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
        let reflection = self.reflection.brdf(wo, wi, n);
        let refraction = self.refraction.brdf(wo, wi, n);
        let factor = reflectance(self.r0, wo, n);
        reflection * (1.0 - factor) + refraction * factor
    }

    fn sample(&self, wi: &Vector3<f32>, n: &Vector3<f32>, rng: &mut SmallRng) -> MaterialSample {
        if rng.gen::<f32>() < reflectance(self.r0, wi, n) {
            self.reflection.sample(wi, n, rng)
        } else {
            self.refraction.sample(wi, n, rng)
        }
    }
}

struct BlendMaterial<M1, M2> {
    first: M1,
    second: M2,
    factor: f32,
}

impl<MReflective, MRefractive> Material for BlendMaterial<MReflective, MRefractive>
    where
        MReflective: Material,
        MRefractive: Material,
{
    fn brdf(&self, wo: &Vector3<f32>, wi: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
        let first = self.first.brdf(wo, wi, n);
        let second = self.second.brdf(wo, wi, n);
        first * (1.0 - self.factor) + second * self.factor
    }

    fn sample(&self, wi: &Vector3<f32>, n: &Vector3<f32>, rng: &mut SmallRng) -> MaterialSample {
        if rng.gen::<f32>() < self.factor {
            self.first.sample(wi, n, rng)
        } else {
            self.second.sample(wi, n, rng)
        }
    }
}

