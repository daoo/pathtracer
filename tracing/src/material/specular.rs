use glam::Vec3;

use crate::material::BsdfSample;

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

#[derive(Clone, Copy, Debug)]
pub struct Specular(pub Vec3);

impl Specular {
    pub const ZERO: Self = Self(Vec3::ZERO);

    pub fn sample_wo(self, wi: Vec3, normal: Vec3) -> Vec3 {
        reflect(-wi, normal).normalize()
    }

    pub fn sample(self, wi: Vec3, normal: Vec3, probability: f32) -> BsdfSample {
        let wo = self.sample_wo(wi, normal);
        BsdfSample {
            is_delta: true,
            pdf: probability,
            bsdf: self.0 / probability,
            wo,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_is_delta_reflected_around_normal() {
        let wi = Vec3::new(-0.6, -0.8, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let specular = Specular(Vec3::new(0.3, 0.5, 0.7));

        let sample = specular.sample(wi, normal, 1.0);

        assert_eq!(
            sample,
            BsdfSample {
                is_delta: true,
                pdf: 1.0,
                bsdf: Vec3::new(0.3, 0.5, 0.7),
                wo: Vec3::new(0.6, -0.8, 0.0),
            }
        );
    }
}
