use geometry::{algorithms::TriangleRayIntersection, ray::Ray, triangle::Triangle};
use kdtree::{build::build_kdtree, build_sah::SahKdTreeBuilder, KdTree};
use nalgebra::{UnitVector3, Vector2, Vector3};
use std::{collections::BTreeMap, sync::Arc};
use wavefront::{mtl, obj};

use crate::{
    camera::Camera,
    light::SphericalLight,
    material::{
        BlendMaterial, DiffuseReflectiveMaterial, FresnelBlendMaterial, Material,
        SpecularReflectiveMaterial, SpecularRefractiveMaterial,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct TriangleNormals {
    pub n0: Vector3<f32>,
    pub n1: Vector3<f32>,
    pub n2: Vector3<f32>,
}

impl TriangleNormals {
    pub fn lerp(&self, u: f32, v: f32) -> UnitVector3<f32> {
        UnitVector3::new_normalize((1.0 - (u + v)) * self.n0 + u * self.n1 + v * self.n2)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TriangleTexcoords {
    pub uv0: Vector2<f32>,
    pub uv1: Vector2<f32>,
    pub uv2: Vector2<f32>,
}

pub struct Scene {
    pub kdtree: KdTree,
    pub triangle_normals: Vec<TriangleNormals>,
    pub triangle_texcoords: Vec<TriangleTexcoords>,
    pub triangle_materials: Vec<Arc<dyn Material + Send + Sync>>,
    pub cameras: Vec<Camera>,
    pub lights: Vec<SphericalLight>,
}

fn triangles_from_obj(obj: &obj::Obj) -> Vec<Triangle> {
    obj.chunks
        .iter()
        .flat_map(|chunk| {
            chunk.faces.iter().map(|face| Triangle {
                v0: obj.index_vertex(&face.p0).into(),
                v1: obj.index_vertex(&face.p1).into(),
                v2: obj.index_vertex(&face.p2).into(),
            })
        })
        .collect()
}

fn triangle_normals_from_obj(obj: &obj::Obj) -> Vec<TriangleNormals> {
    obj.chunks
        .iter()
        .flat_map(|chunk| {
            chunk.faces.iter().map(|face| TriangleNormals {
                n0: obj.index_normal(&face.p0).into(),
                n1: obj.index_normal(&face.p1).into(),
                n2: obj.index_normal(&face.p2).into(),
            })
        })
        .collect()
}

fn triangle_texcoords_from_obj(obj: &obj::Obj) -> Vec<TriangleTexcoords> {
    obj.chunks
        .iter()
        .flat_map(|chunk| {
            chunk.faces.iter().map(|face| TriangleTexcoords {
                uv0: obj.index_texcoord(&face.p0).into(),
                uv1: obj.index_texcoord(&face.p1).into(),
                uv2: obj.index_texcoord(&face.p2).into(),
            })
        })
        .collect()
}

fn blend_from_mtl(material: &mtl::Material) -> Arc<dyn Material + Send + Sync> {
    let reflection = DiffuseReflectiveMaterial {
        reflectance: material.diffuse_reflection.into(),
    };
    let refraction = SpecularRefractiveMaterial {
        index_of_refraction: material.index_of_refraction,
    };
    let specular = SpecularReflectiveMaterial {
        reflectance: material.specular_reflection.into(),
    };
    let transparency_blend = BlendMaterial {
        first: refraction,
        second: reflection,
        factor: material.transparency,
    };
    let fresnel_blend = FresnelBlendMaterial {
        reflection: specular,
        refraction: transparency_blend.clone(),
        r0: material.reflection_0_degrees,
    };
    Arc::new(BlendMaterial {
        first: fresnel_blend,
        second: transparency_blend,
        factor: material.reflection_90_degrees,
    })
}

fn triangle_materials_from_obj_and_mtl(
    obj: &obj::Obj,
    mtl: &mtl::Mtl,
) -> Vec<Arc<dyn Material + Send + Sync>> {
    let materials = mtl
        .materials
        .iter()
        .map(|m| (m.name.as_str(), blend_from_mtl(m)))
        .collect::<BTreeMap<_, _>>();
    obj.chunks
        .iter()
        .flat_map(|chunk| {
            chunk
                .faces
                .iter()
                .map(|_| materials[chunk.material.as_str()].clone())
        })
        .collect()
}

fn cameras_from_mtl(mtl: &mtl::Mtl) -> Vec<Camera> {
    mtl.cameras
        .iter()
        .map(|camera| {
            Camera::new(
                &camera.position.into(),
                &camera.target.into(),
                &camera.up.into(),
                camera.fov,
            )
        })
        .collect()
}

fn lights_from_mtl(mtl: &mtl::Mtl) -> Vec<SphericalLight> {
    mtl.lights
        .iter()
        .map(|light| {
            SphericalLight::new(
                light.position.into(),
                light.radius,
                &light.color.into(),
                light.intensity,
            )
        })
        .collect()
}

impl Scene {
    pub fn intersect(
        &self,
        ray: &Ray,
        tmin: f32,
        tmax: f32,
    ) -> Option<(usize, TriangleRayIntersection)> {
        self.kdtree.intersect(ray, tmin, tmax)
    }

    pub fn intersect_any(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        self.intersect(ray, tmin, tmax).is_some()
    }

    pub fn from_wavefront(obj: &obj::Obj, mtl: &mtl::Mtl) -> Scene {
        let builder = SahKdTreeBuilder {
            traverse_cost: 2.0,
            intersect_cost: 1.0,
            empty_factor: 0.8,
            triangles: triangles_from_obj(obj),
        };
        let kdtree = build_kdtree(builder, 20);
        Scene {
            kdtree,
            triangle_normals: triangle_normals_from_obj(obj),
            triangle_texcoords: triangle_texcoords_from_obj(obj),
            triangle_materials: triangle_materials_from_obj_and_mtl(obj, mtl),
            cameras: cameras_from_mtl(mtl),
            lights: lights_from_mtl(mtl),
        }
    }
}
