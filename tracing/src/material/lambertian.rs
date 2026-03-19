use std::f32::consts::FRAC_1_PI;

use glam::{Vec2, Vec3};
use image::Rgb32FImage;
use rand::rngs::SmallRng;

use crate::{material::BsdfSample, sampling::cosine_sample_hemisphere};

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

fn wrap01(x: f32) -> f32 {
    let y = x - x.floor();
    if y == 1.0 { 0.0 } else { y }
}

#[derive(Clone, Debug)]
pub enum Lambertian {
    Color(Vec3),
    Texture(Rgb32FImage),
}

impl Lambertian {
    pub const ZERO: Self = Self::Color(Vec3::ZERO);

    pub fn albedo(&self, uv: Vec2) -> Vec3 {
        match self {
            Lambertian::Color(albedo) => *albedo,
            Lambertian::Texture(texture) => {
                let px = (texture.width() as f32 * wrap01(uv.x)).floor();
                let py = (texture.height() as f32 * wrap01(uv.y)).floor();
                Vec3::from(texture.get_pixel(px as u32, py as u32).0)
            }
        }
    }

    pub fn brdf(&self, uv: Vec2) -> Vec3 {
        self.albedo(uv) * FRAC_1_PI
    }

    pub fn sample_wo(&self, normal: Vec3, rng: &mut SmallRng) -> Vec3 {
        let tangent = perpendicular(normal).normalize();
        let bitangent = normal.cross(tangent);

        let hemisphere_sample = cosine_sample_hemisphere(rng);

        (hemisphere_sample.x * tangent
            + hemisphere_sample.y * bitangent
            + hemisphere_sample.z * normal)
            .normalize()
    }

    pub fn sample(
        &self,
        normal: Vec3,
        uv: Vec2,
        rng: &mut SmallRng,
        probability: f32,
    ) -> BsdfSample {
        let wo = self.sample_wo(normal, rng);
        let cos_theta = wo.dot(normal).max(0.0);
        let brdf = self.brdf(uv);
        BsdfSample {
            is_delta: false,
            pdf: probability * cos_theta * FRAC_1_PI,
            bsdf: brdf,
            wo,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use image::Rgb;

    use std::{f32::consts::PI, iter};

    use rand::SeedableRng;

    #[test]
    fn lambertian_pdf_matches_cosine() {
        let lambertian = Lambertian::Color(Vec3::new(0.2, 0.4, 0.6));
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let mut rng = SmallRng::seed_from_u64(1234u64);

        let actual = lambertian.sample(normal, Vec2::ZERO, &mut rng, 1.0);

        let cos = actual.wo.dot(normal).max(0.0);
        let expected_pdf = cos / PI;
        assert_ulps_eq!(actual.pdf, expected_pdf);
        assert!(cos >= 0.0);
    }

    #[test]
    fn lambertian_throughput_equals_albedo() {
        let albedo = Vec3::new(0.1, 0.2, 0.3);
        let lambertian = Lambertian::Color(albedo);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let uv = Vec2::new(0.3, 0.7);
        let mut rng = SmallRng::seed_from_u64(1234u64);
        let iterations = 1000;

        let sum: Vec3 = iter::repeat_with(|| {
            let actual = lambertian.sample(normal, uv, &mut rng, 1.0);
            let cos = actual.wo.dot(normal).max(0.0);
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
