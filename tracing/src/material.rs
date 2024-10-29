use std::path::Path;

use glam::{Vec2, Vec3};
use image::{GenericImageView, Rgb32FImage};
use rand::{rngs::SmallRng, Rng};
use wavefront::mtl;

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
pub struct Surface {
    pub wi: Vec3,
    pub n: Vec3,
    pub uv: Vec2,
}

impl Surface {
    fn with_normal(&self, n: Vec3) -> Surface {
        Surface {
            wi: self.wi,
            n,
            uv: self.uv,
        }
    }

    fn is_same_hemisphere(&self, wo: &Vec3) -> bool {
        is_same_sign(self.wi.dot(self.n), wo.dot(self.n))
    }

    fn reflectance(&self, r0: f32) -> f32 {
        r0 + (1.0 - r0) * (1.0 - self.wi.dot(self.n).abs()).powi(5)
    }
}

#[derive(Debug, PartialEq)]
pub struct SurfaceSample {
    pub pdf: f32,
    pub brdf: Vec3,
    pub wo: Vec3,
}

fn diffuse_reflective_brdf(texture: &Option<Rgb32FImage>, reflectance: &Vec3, uv: &Vec2) -> Vec3 {
    if let Some(texture) = &texture {
        let px = (texture.width() as f32 * uv.x).floor();
        let py = (texture.height() as f32 * uv.y).floor();
        let reflectance = Vec3::from(unsafe { texture.unsafe_get_pixel(px as u32, py as u32).0 });
        reflectance * std::f32::consts::FRAC_1_PI
    } else {
        reflectance * std::f32::consts::FRAC_1_PI
    }
}

fn diffuse_reflection_sample(
    texture: &Option<Rgb32FImage>,
    reflectance: &Vec3,
    surface_normal: &Vec3,
    uv: &Vec2,
    rng: &mut SmallRng,
) -> SurfaceSample {
    let tangent = perpendicular(surface_normal).normalize();
    let bitangent = surface_normal.cross(tangent);
    let s = cosine_sample_hemisphere(rng);

    let wo = (s.x * tangent + s.y * bitangent + s.z * surface_normal).normalize();
    SurfaceSample {
        pdf: 1.0,
        brdf: diffuse_reflective_brdf(texture, reflectance, uv),
        wo,
    }
}

fn specular_reflection_sample(reflectance: Vec3, surface: &Surface) -> SurfaceSample {
    let wo = (2.0 * surface.wi.dot(surface.n).abs() * surface.n - surface.wi).normalize();
    let pdf = if surface.is_same_hemisphere(&wo) {
        wo.dot(surface.n).abs()
    } else {
        0.0
    };
    SurfaceSample {
        pdf,
        brdf: reflectance,
        wo,
    }
}

fn specular_refractive_sample(index_of_refraction: f32, surface: &Surface) -> SurfaceSample {
    let (eta, n_refracted) = if (-surface.wi).dot(surface.n) < 0.0 {
        (1.0 / index_of_refraction, surface.n)
    } else {
        (index_of_refraction, -surface.n)
    };

    let w = -(-surface.wi).dot(n_refracted) * eta;
    let k = 1.0 + (w - eta) * (w + eta);
    if k < 0.0 {
        return specular_reflection_sample(Vec3::ONE, &surface.with_normal(n_refracted));
    }

    let k = k.sqrt();
    let wo = (-eta * surface.wi + (w - k) * n_refracted).normalize();
    SurfaceSample {
        pdf: 1.0,
        brdf: Vec3::new(1.0, 1.0, 1.0),
        wo,
    }
}

fn mix(x: Vec3, y: Vec3, a: f32) -> Vec3 {
    x * (1.0 - a) + y * a
}

#[derive(Clone, Debug)]
pub struct Material {
    pub diffuse_reflectance: Vec3,
    pub diffuse_texture_reflectance: Option<Rgb32FImage>,
    pub specular_reflectance: Vec3,
    pub index_of_refraction: f32,
    pub reflection_0_degrees: f32,
    pub reflection_90_degrees: f32,
    pub transparency: f32,
}

impl Material {
    pub fn load_from_mtl(image_directory: &Path, material: &mtl::Material) -> Material {
        let texture = (!material.diffuse_map.is_empty()).then(|| {
            image::open(image_directory.join(&material.diffuse_map))
                .unwrap()
                .to_rgb32f()
        });
        Material {
            diffuse_reflectance: material.diffuse_reflection.into(),
            diffuse_texture_reflectance: texture,
            specular_reflectance: material.specular_reflection.into(),
            index_of_refraction: material.index_of_refraction,
            reflection_0_degrees: material.reflection_0_degrees,
            reflection_90_degrees: material.reflection_90_degrees,
            transparency: material.transparency,
        }
    }

    pub fn brdf(&self, surface: &Surface) -> Vec3 {
        let reflection = diffuse_reflective_brdf(
            &self.diffuse_texture_reflectance,
            &self.diffuse_reflectance,
            &surface.uv,
        );
        let transparency_blend = mix(reflection, Vec3::ZERO, self.transparency);
        let fresnel_blend = mix(
            transparency_blend,
            Vec3::ZERO,
            surface.reflectance(self.reflection_0_degrees),
        );
        mix(
            transparency_blend,
            fresnel_blend,
            self.reflection_90_degrees,
        )
    }

    pub fn sample(&self, surface: &Surface, rng: &mut SmallRng) -> SurfaceSample {
        if rng.gen::<f32>() < self.reflection_90_degrees {
            if rng.gen::<f32>() < surface.reflectance(self.reflection_0_degrees) {
                specular_reflection_sample(self.specular_reflectance, surface)
            } else if rng.gen::<f32>() < self.transparency {
                specular_refractive_sample(self.index_of_refraction, surface)
            } else {
                diffuse_reflection_sample(
                    &self.diffuse_texture_reflectance,
                    &self.diffuse_reflectance,
                    &surface.n,
                    &surface.uv,
                    rng,
                )
            }
        } else if rng.gen::<f32>() < self.transparency {
            specular_refractive_sample(self.index_of_refraction, surface)
        } else {
            diffuse_reflection_sample(
                &self.diffuse_texture_reflectance,
                &self.diffuse_reflectance,
                &surface.n,
                &surface.uv,
                rng,
            )
        }
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
        let surface = Surface { wi, n, uv };

        let actual = specular_refractive_sample(index_of_refraction, &surface);

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
        let surface = Surface { wi, n, uv };

        let actual = specular_refractive_sample(index_of_refraction, &surface);

        let n1 = index_of_refraction;
        let n2 = 1.0;
        let theta1 = wi.dot(n).acos();
        let theta2 = actual.wo.dot(n).acos();
        assert_approx_eq!(n1 * theta1.sin(), n2 * theta2.sin(), 2e-7);
        assert!(actual.wo.y > 0.0);
    }
}
