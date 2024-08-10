use glam::{Vec2, Vec3};
use image::Rgb32FImage;
use rand::{rngs::SmallRng, Rng};
use scene::material::Material;

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

fn diffuse_reflective_brdf(
    texture: &Option<Rgb32FImage>,
    reflectance: Vec3,
    outgoing: &OutgoingRay,
) -> Vec3 {
    if let Some(texture) = &texture {
        let px = (texture.width() as f32 * outgoing.uv.x).floor();
        let py = (texture.height() as f32 * outgoing.uv.y).floor();
        let reflectance: Vec3 = texture[(px as u32, py as u32)].0.into();
        reflectance * std::f32::consts::FRAC_1_PI
    } else {
        reflectance * std::f32::consts::FRAC_1_PI
    }
}

fn diffuse_reflection_sample(
    texture: &Option<Rgb32FImage>,
    reflectance: Vec3,
    incoming: &IncomingRay,
    rng: &mut SmallRng,
) -> MaterialSample {
    let tangent = perpendicular(&incoming.n).normalize();
    let bitangent = incoming.n.cross(tangent);
    let s = cosine_sample_hemisphere(rng);

    let wo = (s.x * tangent + s.y * bitangent + s.z * incoming.n).normalize();
    MaterialSample {
        pdf: 1.0,
        brdf: diffuse_reflective_brdf(texture, reflectance, &incoming.with_wo(wo)),
        wo,
    }
}

fn specular_reflection_sample(reflectance: Vec3, incoming: &IncomingRay) -> MaterialSample {
    let wo = (2.0 * incoming.wi.dot(incoming.n).abs() * incoming.n - incoming.wi).normalize();
    let outgoing = incoming.with_wo(wo);
    let pdf = if outgoing.is_same_hemisphere() {
        wo.dot(outgoing.n).abs()
    } else {
        0.0
    };
    MaterialSample {
        pdf,
        brdf: reflectance,
        wo,
    }
}

fn specular_refractive_sample(index_of_refraction: f32, incoming: &IncomingRay) -> MaterialSample {
    let (eta, n_refracted) = if (-incoming.wi).dot(incoming.n) < 0.0 {
        (1.0 / index_of_refraction, incoming.n)
    } else {
        (index_of_refraction, -incoming.n)
    };

    let w = -(-incoming.wi).dot(n_refracted) * eta;
    let k = 1.0 + (w - eta) * (w + eta);
    if k < 0.0 {
        return specular_reflection_sample(Vec3::ONE, &incoming.with_normal(n_refracted));
    }

    let k = k.sqrt();
    let wo = (-eta * incoming.wi + (w - k) * n_refracted).normalize();
    MaterialSample {
        pdf: 1.0,
        brdf: Vec3::new(1.0, 1.0, 1.0),
        wo,
    }
}

fn mix(x: Vec3, y: Vec3, a: f32) -> Vec3 {
    x * (1.0 - a) + y * a
}

pub fn material_brdf(material: &Material, outgoing: &OutgoingRay) -> Vec3 {
    let reflection = diffuse_reflective_brdf(
        &material.diffuse_texture_reflectance,
        material.diffuse_reflectance,
        outgoing,
    );
    let transparency_blend = mix(reflection, Vec3::ZERO, material.transparency);
    let fresnel_blend = mix(
        transparency_blend,
        Vec3::ZERO,
        outgoing
            .as_incoming()
            .reflectance(material.reflection_0_degrees),
    );
    mix(
        transparency_blend,
        fresnel_blend,
        material.reflection_90_degrees,
    )
}

pub fn material_sample(
    material: &Material,
    incoming: &IncomingRay,
    rng: &mut SmallRng,
) -> MaterialSample {
    if rng.gen::<f32>() < material.reflection_90_degrees {
        if rng.gen::<f32>() < incoming.reflectance(material.reflection_0_degrees) {
            specular_reflection_sample(material.specular_reflectance, incoming)
        } else if rng.gen::<f32>() < material.transparency {
            specular_refractive_sample(material.index_of_refraction, incoming)
        } else {
            diffuse_reflection_sample(
                &material.diffuse_texture_reflectance,
                material.diffuse_reflectance,
                incoming,
                rng,
            )
        }
    } else if rng.gen::<f32>() < material.transparency {
        specular_refractive_sample(material.index_of_refraction, incoming)
    } else {
        diffuse_reflection_sample(
            &material.diffuse_texture_reflectance,
            material.diffuse_reflectance,
            incoming,
            rng,
        )
    }
}

#[cfg(test)]
mod specular_refractive_material_tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn refraction_into_medium() {
        let index_of_refraction = 1.5;
        let wi = Vec3::new(-1.0, 2.0, 0.0).normalize();
        let n = Vec3::new(0.0, 1.0, 0.0);
        let uv = Vec2::ZERO;
        let incoming = IncomingRay { wi, n, uv };

        let actual = specular_refractive_sample(index_of_refraction, &incoming);

        let n1 = 1.0;
        let n2 = index_of_refraction;
        let theta1 = wi.dot(n).acos();
        let theta2 = actual.wo.dot(-n).acos();
        assert_approx_eq!(n1 * theta1.sin(), n2 * theta2.sin(), 2e-7);
        assert!(actual.wo.y < 0.0);
    }

    #[test]
    fn reflection_out_of_medium() {
        let index_of_refraction = 1.5;
        let wi = Vec3::new(-1.0, -2.0, 0.0).normalize();
        let n = Vec3::new(0.0, 1.0, 0.0);
        let uv = Vec2::ZERO;
        let incoming = IncomingRay { wi, n, uv };

        let actual = specular_refractive_sample(index_of_refraction, &incoming);

        let n1 = index_of_refraction;
        let n2 = 1.0;
        let theta1 = wi.dot(n).acos();
        let theta2 = actual.wo.dot(n).acos();
        assert_approx_eq!(n1 * theta1.sin(), n2 * theta2.sin(), 2e-7);
        assert!(actual.wo.y > 0.0);
    }
}
