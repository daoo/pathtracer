use std::{f32::consts::FRAC_1_PI, path::Path};

use glam::{Vec2, Vec3};
use image::{ConvertColorOptions, metadata::Cicp};
use rand::{RngExt, rngs::SmallRng};
use wavefront::mtl;

use crate::{material::albedo::AlbedoSource, sampling::cosine_sample_hemisphere};

pub mod albedo;

fn luminance(c: Vec3) -> f32 {
    // Rec.709 / sRGB linear luminance
    0.2126 * c.x + 0.7152 * c.y + 0.0722 * c.z
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn perpendicular(v: Vec3) -> Vec3 {
    let vx = v.x.abs();
    let vy = v.y.abs();
    let vz = v.z.abs();
    if vx < vy && vx < vz {
        Vec3::new(0.0, -v.z, v.y)
    } else if vy < vz {
        Vec3::new(-v.z, 0.0, v.x)
    } else {
        Vec3::new(-v.y, v.x, 0.0)
    }
}

fn schlicks_approximation(r0: Vec3, wi: Vec3, n: Vec3) -> Vec3 {
    let cos_theta = wi.dot(n).max(0.0);
    let t = (1.0 - cos_theta).powi(5);
    r0 + (1.0 - r0) * t
}

#[derive(Debug)]
pub struct Surface {
    pub wi: Vec3,
    pub n: Vec3,
    pub uv: Vec2,
}

#[derive(Debug, PartialEq)]
pub struct BsdfSample {
    pub is_delta: bool,
    pub pdf: f32,
    pub bsdf: Vec3,
    pub wo: Vec3,
}

impl BsdfSample {
    fn zero(normal: Vec3) -> Self {
        Self {
            is_delta: true,
            pdf: 0.0,
            bsdf: Vec3::ZERO,
            wo: normal,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Material {
    pub albedo: AlbedoSource,
    pub schlick_f0: Vec3,
    pub transmission: f32,
    pub ior: f32,
}

fn sample_specular(surface: &Surface, color: Vec3, probability: f32) -> BsdfSample {
    let wo = reflect(-surface.wi, surface.n).normalize();
    BsdfSample {
        is_delta: true,
        pdf: probability,
        bsdf: color,
        wo,
    }
}

fn sample_refraction(
    surface: &Surface,
    ior: f32,
    diffuse: Vec3,
    transmitted_diffuse: Vec3,
    probability: f32,
) -> BsdfSample {
    let is_entering = surface.wi.dot(surface.n) < 0.0;
    let n1 = if is_entering { 1.0 } else { ior };
    let n2 = if is_entering { ior } else { 1.0 };
    let eta = n1 / n2;
    let normal = if is_entering { surface.n } else { -surface.n };
    let incoming = -surface.wi;
    let cos_theta_i = incoming.dot(normal);
    let sin2_theta_t = eta * eta * (1.0 - cos_theta_i * cos_theta_i);
    if sin2_theta_t >= 1.0 {
        // Fallback to reflection if refraction is impossible
        return sample_specular(surface, diffuse, probability);
    }
    let cos_theta_t = (1.0 - sin2_theta_t).sqrt();
    let wo = (eta * incoming + (eta * cos_theta_i - cos_theta_t) * normal).normalize();
    let eta_scale = (n2 * n2) / (n1 * n1);
    BsdfSample {
        is_delta: true,
        pdf: probability,
        bsdf: transmitted_diffuse * eta_scale,
        wo,
    }
}

fn sample_diffuse(
    surface: &Surface,
    rng: &mut SmallRng,
    transmitted_diffuse: Vec3,
    probability: f32,
) -> BsdfSample {
    let tangent = perpendicular(surface.n).normalize();
    let bitangent = surface.n.cross(tangent);

    let hemisphere_sample = cosine_sample_hemisphere(rng);

    let wo = (hemisphere_sample.x * tangent
        + hemisphere_sample.y * bitangent
        + hemisphere_sample.z * surface.n)
        .normalize();
    let cos_theta = wo.dot(surface.n).max(0.0);
    BsdfSample {
        is_delta: false,
        pdf: probability * cos_theta * FRAC_1_PI,
        bsdf: transmitted_diffuse * FRAC_1_PI,
        wo,
    }
}

impl Material {
    pub fn load_from_mtl(image_directory: &Path, material: &mtl::Material) -> Self {
        let albedo = if material.diffuse_map.is_empty() {
            AlbedoSource::Color(material.diffuse_reflection.into())
        } else {
            let mut image = image::open(image_directory.join(&material.diffuse_map)).unwrap();
            image
                .convert_color_space(
                    Cicp::SRGB_LINEAR,
                    ConvertColorOptions::default(),
                    image::ColorType::Rgba32F,
                )
                .unwrap();
            AlbedoSource::Texture(image.into_rgb32f())
        };
        let f0_dielectric =
            ((material.index_of_refraction - 1.0) / (material.index_of_refraction + 1.0)).powi(2);
        let schlick_f0 = Vec3::splat(f0_dielectric)
            .lerp(material.specular_reflection.into(), material.metalness);
        let transmission = material.transparency;
        let ior = material.index_of_refraction;
        Self {
            albedo,
            schlick_f0,
            transmission,
            ior,
        }
    }

    pub fn sample(&self, surface: &Surface, rng: &mut SmallRng) -> BsdfSample {
        let f = schlicks_approximation(self.schlick_f0, surface.wi, surface.n);
        let diffuse = self.albedo.get(surface.uv);
        let transmitted_diffuse = (1.0 - f) * diffuse;
        let specular_strength = luminance(f);
        let diffuse_strength = luminance(transmitted_diffuse) * (1.0 - self.transmission);
        let refraction_strength = luminance(transmitted_diffuse) * self.transmission;
        let total_strength = specular_strength + diffuse_strength + refraction_strength;
        if total_strength <= 0.0 {
            return BsdfSample::zero(surface.n);
        }
        let p_specular = specular_strength / total_strength;
        let p_diffuse = diffuse_strength / total_strength;
        let p_refraction = refraction_strength / total_strength;
        let r = rng.random::<f32>();
        if p_specular > 0.0 && r < p_specular {
            return sample_specular(surface, f, p_specular);
        } else if r < p_specular + p_refraction {
            return sample_refraction(surface, self.ior, f, transmitted_diffuse, p_refraction);
        } else if p_diffuse > 0.0 {
            return sample_diffuse(surface, rng, transmitted_diffuse, p_diffuse);
        }

        BsdfSample::zero(surface.n)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;
    use rand::SeedableRng;

    use super::*;

    #[test]
    fn sample_returns_zero_when_no_lobes() {
        let surface = Surface {
            wi: Vec3::new(0.0, 1.0, 0.0),
            n: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::ZERO,
        };
        let material = Material {
            albedo: AlbedoSource::ZERO,
            schlick_f0: Vec3::ZERO,
            transmission: 0.0,
            ior: 1.0,
        };
        let mut rng = SmallRng::seed_from_u64(1234);

        let sample = material.sample(&surface, &mut rng);

        assert_eq!(
            sample,
            BsdfSample {
                is_delta: true,
                pdf: 0.0,
                bsdf: Vec3::ZERO,
                wo: Vec3::new(0.0, 1.0, 0.0),
            }
        );
    }

    #[test]
    fn sample_specular_only() {
        let surface = Surface {
            wi: Vec3::new(0.8, 0.6, 0.0),
            n: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::ZERO,
        };
        let material = Material {
            albedo: AlbedoSource::ZERO,
            schlick_f0: Vec3::new(0.2, 0.4, 0.6),
            transmission: 0.0,
            ior: 1.0,
        };
        let mut rng = SmallRng::seed_from_u64(1234);

        let actual = material.sample(&surface, &mut rng);

        assert!(actual.is_delta);
        assert_ulps_eq!(actual.pdf, 1.0);
        assert_ulps_eq!(actual.bsdf, Vec3::new(0.208192, 0.406144, 0.604096));
        assert_ulps_eq!(actual.wo, Vec3::new(-0.8, 0.6, 0.0));
    }

    #[test]
    fn sample_diffuse_only() {
        let surface = Surface {
            wi: Vec3::new(0.0, 1.0, 0.0),
            n: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::ZERO,
        };
        let albedo = Vec3::new(0.2, 0.4, 0.6);
        let material = Material {
            albedo: AlbedoSource::Color(albedo),
            schlick_f0: Vec3::ZERO,
            transmission: 0.0,
            ior: 1.0,
        };
        let rng = || SmallRng::seed_from_u64(1);

        let actual = material.sample(&surface, &mut rng());
        assert!(!actual.is_delta);
        let cos_theta = actual.wo.dot(surface.n).max(0.0);
        assert_ulps_eq!(actual.pdf, cos_theta * FRAC_1_PI);
        assert_ulps_eq!(actual.bsdf, albedo * FRAC_1_PI);
        assert!(cos_theta > 0.0);
        assert_ulps_eq!(actual.wo.length(), 1.0);
    }

    #[test]
    fn sample_mixed_lobes_diffuse_branch() {
        let surface = Surface {
            wi: Vec3::new(0.0, 1.0, 0.0),
            n: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::ZERO,
        };
        // Luminance is 1/3 so diffuse/specular weights are exactly 0.5 when f = 0.25.
        let albedo = Vec3::new(0.2, 0.34604772, 0.6);
        let material = Material {
            albedo: AlbedoSource::Color(albedo),
            schlick_f0: Vec3::splat(0.25),
            transmission: 0.0,
            ior: 1.0,
        };
        let f = material.schlick_f0;
        let p_specular = 0.5;
        let seed = (0u64..1024)
            .find(|&seed| SmallRng::seed_from_u64(seed).random::<f32>() >= p_specular)
            .expect("seed for diffuse branch");
        let mut rng = SmallRng::seed_from_u64(seed);
        let actual = material.sample(&surface, &mut rng);

        let cos_theta = actual.wo.dot(surface.n).max(0.0);
        assert!(!actual.is_delta);
        assert!(actual.pdf > 0.0);
        assert!(actual.pdf < cos_theta * FRAC_1_PI);
        assert_ulps_eq!(actual.bsdf, (1.0 - f) * albedo * FRAC_1_PI);
        assert!(cos_theta > 0.0);
    }

    #[test]
    fn sample_mixed_lobes_specular_branch() {
        let surface = Surface {
            wi: Vec3::new(0.0, 1.0, 0.0),
            n: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::new(0.3, 0.7),
        };
        let albedo = Vec3::new(0.2, 0.34604772, 0.6);
        let material = Material {
            albedo: AlbedoSource::Color(albedo),
            schlick_f0: Vec3::splat(0.25),
            transmission: 0.0,
            ior: 1.0,
        };
        let p_specular = 0.5;
        let seed = (0u64..1024)
            .find(|&seed| SmallRng::seed_from_u64(seed).random::<f32>() < p_specular)
            .expect("seed for specular branch");
        let mut rng = SmallRng::seed_from_u64(seed);
        let actual = material.sample(&surface, &mut rng);

        assert!(actual.is_delta);
        assert!(actual.pdf > 0.0);
        assert!(actual.pdf < 1.0);
        assert_ulps_eq!(actual.bsdf, material.schlick_f0);
        assert_ulps_eq!(actual.wo, Vec3::new(0.0, 1.0, 0.0));
    }
}
