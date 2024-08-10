use glam::Vec3;
use image::Rgb32FImage;

#[derive(Clone, Debug)]
pub struct DiffuseReflectiveMaterial {
    pub reflectance: Vec3,
    pub texture: Option<Rgb32FImage>,
}

#[derive(Clone, Debug)]
pub struct SpecularReflectiveMaterial {
    pub reflectance: Vec3,
}

#[derive(Clone, Debug)]
pub struct SpecularRefractiveMaterial {
    pub index_of_refraction: f32,
}

#[derive(Clone, Debug)]
pub struct FresnelBlendMaterial<M1, M2> {
    pub reflection: M1,
    pub refraction: M2,
    pub r0: f32,
}

#[derive(Clone, Debug)]
pub struct BlendMaterial<M1, M2> {
    pub first: M1,
    pub second: M2,
    pub factor: f32,
}

pub type MaterialModel = BlendMaterial<
    FresnelBlendMaterial<
        SpecularReflectiveMaterial,
        BlendMaterial<SpecularRefractiveMaterial, DiffuseReflectiveMaterial>,
    >,
    BlendMaterial<SpecularRefractiveMaterial, DiffuseReflectiveMaterial>,
>;
