use glam::Vec3;
use image::Rgb32FImage;

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
