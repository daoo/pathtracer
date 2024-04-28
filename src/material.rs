use nalgebra::{RealField, UnitVector3, Vector3};
use rand::{rngs::SmallRng, Rng};
use scene::material::{
    BlendMaterial, DiffuseReflectiveMaterial, FresnelBlendMaterial, MaterialSample,
    SpecularReflectiveMaterial, SpecularRefractiveMaterial,
};

use crate::sampling::cosine_sample_hemisphere;

fn perpendicular(v: &Vector3<f32>) -> Vector3<f32> {
    if v.x.abs() < v.y.abs() {
        Vector3::new(0., -v.z, v.y)
    } else {
        Vector3::new(-v.z, 0., v.x)
    }
}

fn is_same_sign(a: f32, b: f32) -> bool {
    a.signum() == b.signum()
}

fn is_same_hemisphere(wi: &Vector3<f32>, wo: &Vector3<f32>, n: &UnitVector3<f32>) -> bool {
    is_same_sign(wi.dot(n), wo.dot(n))
}

pub trait Material {
    fn brdf(&self, wi: &Vector3<f32>, wo: &Vector3<f32>, n: &UnitVector3<f32>) -> Vector3<f32>;

    fn sample(&self, wi: &Vector3<f32>, n: &UnitVector3<f32>, rng: &mut SmallRng)
        -> MaterialSample;
}

impl Material for DiffuseReflectiveMaterial {
    fn brdf(&self, _: &Vector3<f32>, _: &Vector3<f32>, _: &UnitVector3<f32>) -> Vector3<f32> {
        self.reflectance * f32::frac_1_pi()
    }

    fn sample(
        &self,
        wi: &Vector3<f32>,
        n: &UnitVector3<f32>,
        rng: &mut SmallRng,
    ) -> MaterialSample {
        let tangent = perpendicular(n).normalize();
        let bitangent = n.cross(&tangent);
        let s = cosine_sample_hemisphere(rng);

        let wo = UnitVector3::new_normalize(s.x * tangent + s.y * bitangent + s.z * n.into_inner());
        MaterialSample {
            pdf: 1.0,
            brdf: self.brdf(wi, &wo, n),
            wo,
        }
    }
}

impl Material for SpecularReflectiveMaterial {
    fn brdf(&self, _: &Vector3<f32>, _: &Vector3<f32>, _: &UnitVector3<f32>) -> Vector3<f32> {
        Vector3::zeros()
    }

    fn sample(&self, wi: &Vector3<f32>, n: &UnitVector3<f32>, _: &mut SmallRng) -> MaterialSample {
        let wo = UnitVector3::new_normalize(2.0 * wi.dot(n).abs() * n.into_inner() - wi);
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
    fn brdf(&self, _: &Vector3<f32>, _: &Vector3<f32>, _: &UnitVector3<f32>) -> Vector3<f32> {
        Vector3::zeros()
    }

    fn sample(
        &self,
        wi: &Vector3<f32>,
        n: &UnitVector3<f32>,
        rng: &mut SmallRng,
    ) -> MaterialSample {
        let (eta, n_refracted) = if (-wi).dot(n) < 0.0 {
            (1.0 / self.index_of_refraction, *n)
        } else {
            (self.index_of_refraction, -*n)
        };

        let w = -(-wi).dot(&n_refracted) * eta;
        let k = 1.0 + (w - eta) * (w + eta);
        if k < 0.0 {
            const TOTAL_INTERNAL_REFLECTION: SpecularReflectiveMaterial =
                SpecularReflectiveMaterial {
                    reflectance: Vector3::new(1.0, 1.0, 1.0),
                };
            return TOTAL_INTERNAL_REFLECTION.sample(wi, &n_refracted, rng);
        }

        let k = k.sqrt();
        let wo = UnitVector3::new_normalize(-eta * wi + (w - k) * *n_refracted);
        MaterialSample {
            pdf: 1.0,
            brdf: Vector3::new(1., 1., 1.),
            wo,
        }
    }
}

#[cfg(test)]
mod specular_refractive_material_tests {
    use assert_approx_eq::assert_approx_eq;
    use rand::SeedableRng;

    use super::*;

    #[test]
    fn refraction_into_medium() {
        let material = SpecularRefractiveMaterial {
            index_of_refraction: 1.5,
        };
        let wi = Vector3::new(-1.0, 2.0, 0.0).normalize();
        let n = UnitVector3::new_normalize(Vector3::new(0.0, 1.0, 0.0));
        let mut rng = SmallRng::seed_from_u64(0);

        let actual = material.sample(&wi, &n, &mut rng);

        let n1 = 1.0;
        let n2 = material.index_of_refraction;
        let theta1 = wi.dot(&n).acos();
        let theta2 = actual.wo.dot(&-n).acos();
        assert_approx_eq!(n1 * theta1.sin(), n2 * theta2.sin(), 2e-7);
        assert!(actual.wo.y < 0.0);
    }

    #[test]
    fn reflection_out_of_medium() {
        let material = SpecularRefractiveMaterial {
            index_of_refraction: 1.5,
        };
        let wi = Vector3::new(-1.0, -2.0, 0.0).normalize();
        let n = UnitVector3::new_normalize(Vector3::new(0.0, 1.0, 0.0));
        let mut rng = SmallRng::seed_from_u64(0);

        let actual = material.sample(&wi, &n, &mut rng);

        let n1 = material.index_of_refraction;
        let n2 = 1.0;
        let theta1 = wi.dot(&n).acos();
        let theta2 = actual.wo.dot(&n).acos();
        assert_approx_eq!(n1 * theta1.sin(), n2 * theta2.sin(), 2e-7);
        assert!(actual.wo.y > 0.0);
    }
}

fn reflectance(r0: f32, wi: &Vector3<f32>, n: &Vector3<f32>) -> f32 {
    r0 + (1.0 - r0) * (1.0 - wi.dot(n).abs()).powf(5.0)
}

fn mix(x: Vector3<f32>, y: Vector3<f32>, a: f32) -> Vector3<f32> {
    x * (1.0 - a) + y * a
}

impl<M1, M2> Material for FresnelBlendMaterial<M1, M2>
where
    M1: Material,
    M2: Material,
{
    fn brdf(&self, wi: &Vector3<f32>, wo: &Vector3<f32>, n: &UnitVector3<f32>) -> Vector3<f32> {
        mix(
            self.refraction.brdf(wi, wo, n),
            self.reflection.brdf(wi, wo, n),
            reflectance(self.r0, wi, n),
        )
    }

    fn sample(
        &self,
        wi: &Vector3<f32>,
        n: &UnitVector3<f32>,
        rng: &mut SmallRng,
    ) -> MaterialSample {
        if rng.gen::<f32>() < reflectance(self.r0, wi, n) {
            self.reflection.sample(wi, n, rng)
        } else {
            self.refraction.sample(wi, n, rng)
        }
    }
}

impl<M1, M2> Material for BlendMaterial<M1, M2>
where
    M1: Material,
    M2: Material,
{
    fn brdf(&self, wi: &Vector3<f32>, wo: &Vector3<f32>, n: &UnitVector3<f32>) -> Vector3<f32> {
        mix(
            self.second.brdf(wi, wo, n),
            self.first.brdf(wi, wo, n),
            self.factor,
        )
    }

    fn sample(
        &self,
        wi: &Vector3<f32>,
        n: &UnitVector3<f32>,
        rng: &mut SmallRng,
    ) -> MaterialSample {
        if rng.gen::<f32>() < self.factor {
            self.first.sample(wi, n, rng)
        } else {
            self.second.sample(wi, n, rng)
        }
    }
}
