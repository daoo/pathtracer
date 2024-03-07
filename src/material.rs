use crate::sampling::*;
use nalgebra::{vector, RealField, Vector3};
use rand::rngs::SmallRng;
use rand::Rng;
use std::sync::Arc;

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

#[derive(Debug)]
pub struct DiffuseReflectiveMaterial {
    pub reflectance: Vector3<f32>,
}

#[derive(Debug)]
pub struct SpecularReflectiveMaterial {
    pub reflectance: Vector3<f32>,
}

#[derive(Debug)]
pub struct SpecularRefractiveMaterial {
    pub index_of_refraction: f32,
}

pub struct FresnelBlendMaterial {
    pub reflection: Arc<dyn Material>,
    pub refraction: Arc<dyn Material>,
    pub r0: f32,
}

impl FresnelBlendMaterial {
    pub fn new_approx(
        reflection: Arc<dyn Material>,
        refraction: Arc<dyn Material>,
        r0: f32,
    ) -> Arc<dyn Material> {
        Arc::new(FresnelBlendMaterial {
            reflection,
            refraction,
            r0,
        })
    }
}

pub struct BlendMaterial {
    pub first: Arc<dyn Material>,
    pub second: Arc<dyn Material>,
    pub factor: f32,
}

impl BlendMaterial {
    pub fn new_approx(
        first: Arc<dyn Material>,
        second: Arc<dyn Material>,
        factor: f32,
    ) -> Arc<dyn Material> {
        Arc::new(BlendMaterial {
            first,
            second,
            factor,
        })
    }
}

// TODO: Don't really understand the Arc + Send + Sync stuff.
pub trait Material: Send + Sync {
    fn brdf(&self, wo: &Vector3<f32>, wi: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32>;

    fn sample(&self, wi: &Vector3<f32>, n: &Vector3<f32>, rng: &mut SmallRng) -> MaterialSample;
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
        MaterialSample {
            pdf: s.norm(),
            brdf: self.brdf(wi, &wo, n),
            wo,
        }
    }
}

impl Material for SpecularReflectiveMaterial {
    fn brdf(&self, _: &Vector3<f32>, _: &Vector3<f32>, _: &Vector3<f32>) -> Vector3<f32> {
        Vector3::zeros()
    }

    fn sample(&self, wi: &Vector3<f32>, n: &Vector3<f32>, _: &mut SmallRng) -> MaterialSample {
        let wo = (2.0 * wi.dot(n).abs() * n - wi).normalize();
        let pdf = if is_same_hemisphere(wi, &wo, n) {
            wo.dot(n).abs()
        } else {
            0.0
        };
        MaterialSample {
            pdf,
            brdf: self.reflectance,
            wo,
        }
    }
}

impl Material for SpecularRefractiveMaterial {
    fn brdf(&self, _: &Vector3<f32>, _: &Vector3<f32>, _: &Vector3<f32>) -> Vector3<f32> {
        Vector3::zeros()
    }

    fn sample(&self, wi: &Vector3<f32>, n: &Vector3<f32>, rng: &mut SmallRng) -> MaterialSample {
        let a = (-wi).dot(n);
        let eta = if a < 0.0 {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let n = if a < 0.0 { *n } else { -n };

        let w = -a * eta;
        let k = 1.0 + (w - eta) * (w + eta);
        if k < 0.0 {
            const TOTAL_INTERNAL_REFLECTION: SpecularReflectiveMaterial =
                SpecularReflectiveMaterial {
                    reflectance: vector![1.0, 1.0, 1.0],
                };
            return TOTAL_INTERNAL_REFLECTION.sample(wi, &n, rng);
        }

        let k = k.sqrt();
        let wo = (-eta * wi + (w - k) * n).normalize();
        MaterialSample {
            pdf: 1.0,
            brdf: vector![1., 1., 1.],
            wo,
        }
    }
}

fn reflectance(r0: f32, wo: &Vector3<f32>, n: &Vector3<f32>) -> f32 {
    r0 + (1.0 - r0) * (1.0 - wo.dot(n).abs()).powf(5.0)
}

fn mix(x: Vector3<f32>, y: Vector3<f32>, a: f32) -> Vector3<f32> {
    x * (1.0 - a) + y * a
}

impl Material for FresnelBlendMaterial {
    fn brdf(&self, wo: &Vector3<f32>, wi: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
        mix(
            self.refraction.brdf(wo, wi, n),
            self.reflection.brdf(wo, wi, n),
            reflectance(self.r0, wo, n),
        )
    }

    fn sample(&self, wi: &Vector3<f32>, n: &Vector3<f32>, rng: &mut SmallRng) -> MaterialSample {
        if rng.gen::<f32>() < reflectance(self.r0, wi, n) {
            self.reflection.sample(wi, n, rng)
        } else {
            self.refraction.sample(wi, n, rng)
        }
    }
}

impl Material for BlendMaterial {
    fn brdf(&self, wo: &Vector3<f32>, wi: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
        mix(
            self.second.brdf(wo, wi, n),
            self.first.brdf(wo, wi, n),
            self.factor,
        )
    }

    fn sample(&self, wi: &Vector3<f32>, n: &Vector3<f32>, rng: &mut SmallRng) -> MaterialSample {
        if rng.gen::<f32>() < self.factor {
            self.first.sample(wi, n, rng)
        } else {
            self.second.sample(wi, n, rng)
        }
    }
}
