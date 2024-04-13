use geometry::{intersection::RayIntersection, ray::Ray, triangle::Triangle};
use kdtree::{build::build_kdtree, build_sah::SahKdTreeBuilder, KdTree};
use nalgebra::{UnitVector3, Vector2, Vector3};
use std::{collections::BTreeMap, ops::RangeInclusive, sync::Arc};
use wavefront::{mtl, obj};

use crate::{
    camera::Camera,
    light::SphericalLight,
    material::{
        BlendMaterial, DiffuseReflectiveMaterial, FresnelBlendMaterial, Material,
        SpecularReflectiveMaterial, SpecularRefractiveMaterial,
    },
};

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TriangleTexcoords {
    pub uv0: Vector2<f32>,
    pub uv1: Vector2<f32>,
    pub uv2: Vector2<f32>,
}

pub struct TriangleData {
    pub triangle: Triangle,
    pub normals: TriangleNormals,
    pub texcoords: TriangleTexcoords,
    pub material: Arc<dyn Material + Send + Sync>,
}

pub struct Scene {
    pub kdtree: KdTree,
    pub triangle_data: Vec<TriangleData>,
    pub cameras: Vec<Camera>,
    pub lights: Vec<SphericalLight>,
    pub environment: Vector3<f32>,
}

fn collect_triangle_data(obj: &obj::Obj, mtl: &mtl::Mtl) -> Vec<TriangleData> {
    let materials = materials_from_mtl(mtl);
    obj.chunks
        .iter()
        .flat_map(|chunk| {
            chunk.faces.iter().map(|face| {
                let triangle = Triangle {
                    v0: obj.index_vertex(&face.p0).into(),
                    v1: obj.index_vertex(&face.p1).into(),
                    v2: obj.index_vertex(&face.p2).into(),
                };
                let normals = TriangleNormals {
                    n0: obj.index_normal(&face.p0).into(),
                    n1: obj.index_normal(&face.p1).into(),
                    n2: obj.index_normal(&face.p2).into(),
                };
                let texcoords = TriangleTexcoords {
                    uv0: obj.index_texcoord(&face.p0).into(),
                    uv1: obj.index_texcoord(&face.p1).into(),
                    uv2: obj.index_texcoord(&face.p2).into(),
                };
                TriangleData {
                    triangle,
                    normals,
                    texcoords,
                    material: materials[chunk.material.as_str()].clone(),
                }
            })
        })
        .collect::<Vec<_>>()
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

fn materials_from_mtl(mtl: &mtl::Mtl) -> BTreeMap<&str, Arc<dyn Material + Send + Sync>> {
    mtl.materials
        .iter()
        .map(|m| (m.name.as_str(), blend_from_mtl(m)))
        .collect::<BTreeMap<_, _>>()
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
        t_range: RangeInclusive<f32>,
    ) -> Option<(u32, RayIntersection)> {
        self.kdtree.intersect(ray, t_range)
    }

    pub fn intersect_any(&self, ray: &Ray, t_range: RangeInclusive<f32>) -> bool {
        self.intersect(ray, t_range).is_some()
    }

    pub fn from_wavefront(
        obj: &obj::Obj,
        mtl: &mtl::Mtl,
        max_depth: u32,
        traverse_cost: f32,
        intersect_cost: f32,
        empty_factor: f32,
    ) -> Scene {
        let triangle_data = collect_triangle_data(obj, mtl);
        let triangles = triangle_data.iter().map(|t| t.triangle).collect::<Vec<_>>();
        let builder = SahKdTreeBuilder {
            traverse_cost,
            intersect_cost,
            empty_factor,
            triangles,
        };
        let kdtree = build_kdtree(builder, max_depth);
        Scene {
            kdtree,
            triangle_data,
            cameras: cameras_from_mtl(mtl),
            lights: lights_from_mtl(mtl),
            environment: Vector3::new(0.8, 0.8, 0.8),
        }
    }
}
