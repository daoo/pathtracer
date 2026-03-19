use std::path::Path;

use glam::{Vec2, Vec3};
use image::{ConvertColorOptions, metadata::Cicp};
use rand::{Rng, rngs::SmallRng};
use wavefront::mtl;

use crate::material::{lambertian::Lambertian, specular::Specular};

pub mod lambertian;
pub mod specular;

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
    pub lambertian: Lambertian,
    pub specular: Specular,
}

impl Material {
    pub fn load_from_mtl(image_directory: &Path, material: &mtl::Material) -> Self {
        let lambertian = if material.diffuse_map.is_empty() {
            Lambertian::Color(material.diffuse_reflection.into())
        } else {
            let mut image = image::open(image_directory.join(&material.diffuse_map)).unwrap();
            image
                .convert_color_space(
                    Cicp::SRGB_LINEAR,
                    ConvertColorOptions::default(),
                    image::ColorType::Rgba32F,
                )
                .unwrap();
            Lambertian::Texture(image.into_rgb32f())
        };
        let specular = Specular(material.specular_reflection.into());
        Self {
            lambertian,
            specular,
        }
    }

    pub fn sample(&self, surface: &Surface, rng: &mut SmallRng) -> BsdfSample {
        let diffuse_strength = self.lambertian.albedo(surface.uv).max_element();
        let specular_strength = self.specular.0.max_element();
        let total_strength = diffuse_strength + specular_strength;
        if total_strength <= 0.0 {
            return BsdfSample::zero(surface.n);
        }
        let p_specular = if total_strength > 0.0 {
            specular_strength / total_strength
        } else {
            0.0
        };
        let p_diffuse = 1.0 - p_specular;
        if p_specular > 0.0 && rng.random::<f32>() < p_specular {
            return self.specular.sample(surface.wi, surface.n, p_specular);
        }
        if p_diffuse > 0.0 {
            return self
                .lambertian
                .sample(surface.n, surface.uv, rng, p_diffuse);
        }

        BsdfSample::zero(surface.n)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_1_PI;

    use approx::assert_ulps_eq;
    use rand::SeedableRng;

    use super::*;

    fn surface() -> Surface {
        Surface {
            wi: Vec3::new(-0.6, -0.8, 0.0),
            n: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::new(0.25, 0.75),
        }
    }

    #[test]
    fn sample_returns_zero_when_no_lobes() {
        let material = Material {
            lambertian: Lambertian::ZERO,
            specular: Specular::ZERO,
        };
        let mut rng = SmallRng::seed_from_u64(1234);

        let sample = material.sample(&surface(), &mut rng);

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
        let material = Material {
            lambertian: Lambertian::ZERO,
            specular: Specular(Vec3::new(0.3, 0.5, 0.7)),
        };
        let mut rng = SmallRng::seed_from_u64(1234);

        let actual = material.sample(&surface(), &mut rng);

        let expected = material.specular.sample(surface().wi, surface().n, 1.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_diffuse_only() {
        let albedo = Vec3::new(0.3, 0.5, 0.7);
        let material = Material {
            lambertian: Lambertian::Color(albedo),
            specular: Specular::ZERO,
        };

        let actual = material.sample(&surface(), &mut SmallRng::seed_from_u64(1234));

        let expected = material.lambertian.sample(
            surface().n,
            surface().uv,
            &mut SmallRng::seed_from_u64(1234),
            1.0,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_mixed_lobes_diffuse_pdf_is_full_mixture() {
        let albedo = Vec3::new(0.2, 0.2, 0.2);
        let specular = Vec3::new(0.8, 0.8, 0.8);
        let material = Material {
            lambertian: Lambertian::Color(albedo),
            specular: Specular(specular),
        };
        let mut rng = SmallRng::seed_from_u64(1);
        assert!(SmallRng::seed_from_u64(1).random::<f32>() >= 0.8);

        let actual = material.sample(&surface(), &mut rng);

        let cos_theta = actual.wo.dot(surface().n);
        assert!(!actual.is_delta);
        assert_ulps_eq!(actual.pdf, 0.2 * cos_theta * FRAC_1_PI);
        assert_eq!(actual.bsdf, albedo * FRAC_1_PI);
        assert!(cos_theta > 0.0);
    }

    #[test]
    fn sample_mixed_lobes_specular_is_delta() {
        let albedo = Vec3::new(0.2, 0.2, 0.2);
        let specular = Vec3::new(0.8, 0.8, 0.8);
        let material = Material {
            lambertian: Lambertian::Color(albedo),
            specular: Specular(specular),
        };
        let mut rng = SmallRng::seed_from_u64(2);
        assert!(SmallRng::seed_from_u64(2).random::<f32>() <= 0.8);

        let actual = material.sample(&surface(), &mut rng);

        assert!(actual.is_delta);
        assert_ulps_eq!(actual.pdf, 0.8);
        assert_eq!(actual.bsdf, Vec3::ONE);
        assert_eq!(actual.wo, Vec3::new(0.6, -0.8, 0.0));
    }
}
