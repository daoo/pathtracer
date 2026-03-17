use std::{f32::consts::FRAC_1_PI, path::Path};

use glam::{Vec2, Vec3};
use image::{ConvertColorOptions, Rgb32FImage, metadata::Cicp};
use rand::rngs::SmallRng;
use wavefront::mtl;

use crate::sampling::cosine_sample_hemisphere;

fn perpendicular(v: &Vec3) -> Vec3 {
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

fn wrap01(x: f32) -> f32 {
    let y = x - x.floor();
    if y == 1.0 { 0.0 } else { y }
}

#[derive(Debug)]
pub struct Surface {
    pub wi: Vec3,
    pub n: Vec3,
    pub uv: Vec2,
}

/// BSDF sample at a surface point.
///
/// Contract:
/// - `brdf` is the lobe value f(wo, wi) (not multiplied by selection probability).
/// - `pdf` is the full-mixture PDF for `wo`, including lobe selection.
/// - Integrator weight: `throughput *= brdf * |dot(wo, n)| / pdf`.
#[derive(Debug, PartialEq)]
pub struct BsdfSample {
    pub is_delta: bool,
    pub pdf: f32,
    pub bsdf: Vec3,
    pub wo: Vec3,
}

#[derive(Clone, Debug)]
pub enum Lambertian {
    Color(Vec3),
    Texture(Rgb32FImage),
}

impl Lambertian {
    fn brdf(&self, uv: Vec2) -> Vec3 {
        let albedo = match self {
            Lambertian::Color(albedo) => *albedo,
            Lambertian::Texture(texture) => {
                let px = (texture.width() as f32 * wrap01(uv.x)).floor();
                let py = (texture.height() as f32 * wrap01(uv.y)).floor();
                Vec3::from(texture.get_pixel(px as u32, py as u32).0)
            }
        };
        albedo * FRAC_1_PI
    }

    fn sample(&self, surface_normal: &Vec3, uv: Vec2, rng: &mut SmallRng) -> BsdfSample {
        // Build basis around n
        let tangent = perpendicular(surface_normal).normalize();
        let bitangent = surface_normal.cross(tangent);

        // Sample in local space
        let s = cosine_sample_hemisphere(rng);

        // Transform into built basis
        let wo = (s.x * tangent + s.y * bitangent + s.z * surface_normal).normalize();

        // Compute cos and pdf
        let cos_theta = wo.dot(*surface_normal).max(0.0);
        let brdf = self.brdf(uv);
        let pdf = cos_theta * FRAC_1_PI;
        BsdfSample {
            is_delta: false,
            pdf,
            bsdf: brdf,
            wo,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Material {
    pub lambertian: Lambertian,
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
        Self { lambertian }
    }

    pub fn sample(&self, surface: &Surface, rng: &mut SmallRng) -> BsdfSample {
        self.lambertian.sample(&surface.n, surface.uv, rng)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;
    use image::Rgb;

    use std::{f32::consts::PI, iter};

    use rand::SeedableRng;

    use super::*;

    #[test]
    fn lambertian_pdf_matches_cosine() {
        let lambertian = Lambertian::Color(Vec3::new(0.2, 0.4, 0.6));
        let surface_normal = Vec3::new(0.0, 1.0, 0.0);
        let uv = Vec2::new(0.3, 0.7);
        let mut rng = SmallRng::seed_from_u64(1234u64);

        let actual = lambertian.sample(&surface_normal, uv, &mut rng);

        let cos = actual.wo.dot(surface_normal).max(0.0);
        let expected_pdf = cos / PI;
        assert_ulps_eq!(actual.pdf, expected_pdf);
        assert!(cos >= 0.0);
    }

    #[test]
    fn lambertian_throughput_equals_albedo() {
        let albedo = Vec3::new(0.1, 0.2, 0.3);
        let lambertian = Lambertian::Color(albedo);
        let surface_normal = Vec3::new(0.0, 1.0, 0.0);
        let uv = Vec2::new(0.3, 0.7);
        let mut rng = SmallRng::seed_from_u64(1234u64);
        let iterations = 1000;

        let sum: Vec3 = iter::repeat_with(|| {
            let actual = lambertian.sample(&surface_normal, uv, &mut rng);
            let cos = actual.wo.dot(surface_normal).max(0.0);
            let weight = actual.bsdf * (cos / actual.pdf);
            weight
        })
        .take(iterations)
        .sum();
        let mean = sum / iterations as f32;

        assert_ulps_eq!(mean, albedo, epsilon = 1e-5);
    }

    #[test]
    fn texture_repeat() {
        let texture = Rgb32FImage::from_fn(2, 2, |x, y| Rgb([x as f32 / 2.0, y as f32 / 2.0, 0.0]));
        let lambertian = Lambertian::Texture(texture);
        let uv = Vec2::new(0.2, 0.3);

        let actual1 = lambertian.brdf(uv);
        let actual2 = lambertian.brdf(uv + Vec2::ONE);

        assert_ulps_eq!(actual1, actual2);
    }
}
