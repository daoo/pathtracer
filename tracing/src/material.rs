use glam::{Vec2, Vec3};
use rand::{rngs::SmallRng, Rng};
use scene::material::{
    BlendMaterial, DiffuseReflectiveMaterial, FresnelBlendMaterial, SpecularReflectiveMaterial,
    SpecularRefractiveMaterial,
};

use crate::sampling::cosine_sample_hemisphere;

fn perpendicular(v: &Vec3) -> Vec3 {
    if v.x.abs() < v.y.abs() {
        Vec3::new(0., -v.z, v.y)
    } else {
        Vec3::new(-v.z, 0., v.x)
    }
}

fn is_same_sign(a: f32, b: f32) -> bool {
    a.signum() == b.signum()
}

#[derive(Debug)]
pub struct IncomingRay {
    pub wi: Vec3,
    pub n: Vec3,
    pub uv: Vec2,
}

impl IncomingRay {
    fn with_normal(&self, n: Vec3) -> IncomingRay {
        IncomingRay {
            wi: self.wi,
            n,
            uv: self.uv,
        }
    }

    fn with_wo(&self, wo: Vec3) -> OutgoingRay {
        OutgoingRay {
            wi: self.wi,
            n: self.n,
            uv: self.uv,
            wo,
        }
    }

    fn reflectance(&self, r0: f32) -> f32 {
        r0 + (1.0 - r0) * (1.0 - self.wi.dot(self.n).abs()).powf(5.0)
    }
}

#[derive(Debug)]
pub struct OutgoingRay {
    pub wi: Vec3,
    pub n: Vec3,
    pub uv: Vec2,
    pub wo: Vec3,
}

impl OutgoingRay {
    fn is_same_hemisphere(&self) -> bool {
        is_same_sign(self.wi.dot(self.n), self.wo.dot(self.n))
    }

    fn as_incoming(&self) -> IncomingRay {
        IncomingRay {
            wi: self.wi,
            n: self.n,
            uv: self.uv,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MaterialSample {
    pub pdf: f32,
    pub brdf: Vec3,
    pub wo: Vec3,
}

pub trait Material {
    fn brdf(&self, outgoing: &OutgoingRay) -> Vec3;

    fn sample(&self, incoming: &IncomingRay, rng: &mut SmallRng) -> MaterialSample;
}

impl Material for DiffuseReflectiveMaterial {
    fn brdf(&self, outgoing: &OutgoingRay) -> Vec3 {
        if let Some(texture) = &self.texture {
            let px = (texture.width() as f32 * outgoing.uv.x).floor();
            let py = (texture.height() as f32 * outgoing.uv.y).floor();
            let reflectance: Vec3 = texture[(px as u32, py as u32)].0.into();
            reflectance * std::f32::consts::FRAC_1_PI
        } else {
            self.reflectance * std::f32::consts::FRAC_1_PI
        }
    }

    fn sample(&self, incoming: &IncomingRay, rng: &mut SmallRng) -> MaterialSample {
        let tangent = perpendicular(&incoming.n).normalize();
        let bitangent = incoming.n.cross(tangent);
        let s = cosine_sample_hemisphere(rng);

        let wo = (s.x * tangent + s.y * bitangent + s.z * incoming.n).normalize();
        MaterialSample {
            pdf: 1.0,
            brdf: self.brdf(&incoming.with_wo(wo)),
            wo,
        }
    }
}

impl Material for SpecularReflectiveMaterial {
    fn brdf(&self, _: &OutgoingRay) -> Vec3 {
        Vec3::ZERO
    }

    fn sample(&self, incoming: &IncomingRay, _: &mut SmallRng) -> MaterialSample {
        let wo = (2.0 * incoming.wi.dot(incoming.n).abs() * incoming.n - incoming.wi).normalize();
        let outgoing = incoming.with_wo(wo);
        let pdf = if outgoing.is_same_hemisphere() {
            wo.dot(outgoing.n).abs()
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
    fn brdf(&self, _: &OutgoingRay) -> Vec3 {
        Vec3::ZERO
    }

    fn sample(&self, incoming: &IncomingRay, rng: &mut SmallRng) -> MaterialSample {
        let (eta, n_refracted) = if (-incoming.wi).dot(incoming.n) < 0.0 {
            (1.0 / self.index_of_refraction, incoming.n)
        } else {
            (self.index_of_refraction, -incoming.n)
        };

        let w = -(-incoming.wi).dot(n_refracted) * eta;
        let k = 1.0 + (w - eta) * (w + eta);
        if k < 0.0 {
            const TOTAL_INTERNAL_REFLECTION: SpecularReflectiveMaterial =
                SpecularReflectiveMaterial {
                    reflectance: Vec3::new(1.0, 1.0, 1.0),
                };
            return TOTAL_INTERNAL_REFLECTION.sample(&incoming.with_normal(n_refracted), rng);
        }

        let k = k.sqrt();
        let wo = (-eta * incoming.wi + (w - k) * n_refracted).normalize();
        MaterialSample {
            pdf: 1.0,
            brdf: Vec3::new(1.0, 1.0, 1.0),
            wo,
        }
    }
}

fn mix(x: Vec3, y: Vec3, a: f32) -> Vec3 {
    x * (1.0 - a) + y * a
}

impl<M1, M2> Material for FresnelBlendMaterial<M1, M2>
where
    M1: Material,
    M2: Material,
{
    fn brdf(&self, outgoing: &OutgoingRay) -> Vec3 {
        mix(
            self.refraction.brdf(outgoing),
            self.reflection.brdf(outgoing),
            outgoing.as_incoming().reflectance(self.r0),
        )
    }

    fn sample(&self, incoming: &IncomingRay, rng: &mut SmallRng) -> MaterialSample {
        if rng.gen::<f32>() < incoming.reflectance(self.r0) {
            self.reflection.sample(incoming, rng)
        } else {
            self.refraction.sample(incoming, rng)
        }
    }
}

impl<M1, M2> Material for BlendMaterial<M1, M2>
where
    M1: Material,
    M2: Material,
{
    fn brdf(&self, outgoing: &OutgoingRay) -> Vec3 {
        mix(
            self.second.brdf(outgoing),
            self.first.brdf(outgoing),
            self.factor,
        )
    }

    fn sample(&self, incoming: &IncomingRay, rng: &mut SmallRng) -> MaterialSample {
        if rng.gen::<f32>() < self.factor {
            self.first.sample(incoming, rng)
        } else {
            self.second.sample(incoming, rng)
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
        let wi = Vec3::new(-1.0, 2.0, 0.0).normalize();
        let n = Vec3::new(0.0, 1.0, 0.0);
        let uv = Vec2::ZERO;
        let incoming = IncomingRay { wi, n, uv };
        let mut rng = SmallRng::seed_from_u64(0);

        let actual = material.sample(&incoming, &mut rng);

        let n1 = 1.0;
        let n2 = material.index_of_refraction;
        let theta1 = wi.dot(n).acos();
        let theta2 = actual.wo.dot(-n).acos();
        assert_approx_eq!(n1 * theta1.sin(), n2 * theta2.sin(), 2e-7);
        assert!(actual.wo.y < 0.0);
    }

    #[test]
    fn reflection_out_of_medium() {
        let material = SpecularRefractiveMaterial {
            index_of_refraction: 1.5,
        };
        let wi = Vec3::new(-1.0, -2.0, 0.0).normalize();
        let n = Vec3::new(0.0, 1.0, 0.0);
        let uv = Vec2::ZERO;
        let incoming = IncomingRay { wi, n, uv };
        let mut rng = SmallRng::seed_from_u64(0);

        let actual = material.sample(&incoming, &mut rng);

        let n1 = material.index_of_refraction;
        let n2 = 1.0;
        let theta1 = wi.dot(n).acos();
        let theta2 = actual.wo.dot(n).acos();
        assert_approx_eq!(n1 * theta1.sin(), n2 * theta2.sin(), 2e-7);
        assert!(actual.wo.y > 0.0);
    }
}
