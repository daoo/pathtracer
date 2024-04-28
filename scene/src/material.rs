use nalgebra::{UnitVector3, Vector3};

#[derive(Debug, PartialEq)]
pub struct MaterialSample {
    pub pdf: f32,
    pub brdf: Vector3<f32>,
    pub wo: UnitVector3<f32>,
}

#[derive(Clone, Debug)]
pub struct DiffuseReflectiveMaterial {
    pub reflectance: Vector3<f32>,
}

#[derive(Clone, Debug)]
pub struct SpecularReflectiveMaterial {
    pub reflectance: Vector3<f32>,
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
