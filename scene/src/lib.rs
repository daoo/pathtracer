use geometry::{geometry::Geometry, intersection::RayIntersection, triangle::Triangle};
use glam::{Vec2, Vec3};
use material::Material;
use std::{collections::BTreeMap, fs::File, io::BufReader, path::Path};
use wavefront::{mtl, obj};

pub mod camera;
pub mod light;
pub mod material;

use crate::{camera::Camera, light::SphericalLight};

#[derive(Clone, Debug, PartialEq)]
struct TriangleNormals {
    n0: Vec3,
    n1: Vec3,
    n2: Vec3,
}

impl TriangleNormals {
    #[inline]
    fn lerp(&self, u: f32, v: f32) -> Vec3 {
        ((1.0 - (u + v)) * self.n0 + u * self.n1 + v * self.n2).normalize()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TriangleTexcoords {
    uv0: Vec2,
    uv1: Vec2,
    uv2: Vec2,
}

impl TriangleTexcoords {
    #[inline]
    fn lerp(&self, u: f32, v: f32) -> Vec2 {
        (1.0 - (u + v)) * self.uv0 + u * self.uv1 + v * self.uv2
    }
}

struct TriangleProperties {
    normals: TriangleNormals,
    texcoords: TriangleTexcoords,
    material_index: usize,
}

pub struct Scene {
    pub geometries: Vec<Geometry>,
    properties: Vec<TriangleProperties>,
    materials: Vec<Material>,
    pub cameras: Vec<Camera>,
    pub lights: Vec<SphericalLight>,
    pub environment: Vec3,
}

fn blend_from_mtl(image_directory: &Path, material: &mtl::Material) -> Material {
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

fn materials_from_mtl<'m>(
    image_directory: &Path,
    mtl: &'m mtl::Mtl,
) -> BTreeMap<&'m str, Material> {
    mtl.materials
        .iter()
        .map(|m| (m.name.as_str(), blend_from_mtl(image_directory, m)))
        .collect::<BTreeMap<_, _>>()
}

fn collect_cameras(mtl: &mtl::Mtl) -> Vec<Camera> {
    mtl.cameras
        .iter()
        .map(|camera| {
            Camera::new(
                camera.position.into(),
                camera.target.into(),
                camera.up.into(),
                camera.fov,
            )
        })
        .collect()
}

fn collect_lights(mtl: &mtl::Mtl) -> Vec<SphericalLight> {
    mtl.lights
        .iter()
        .map(|light| {
            SphericalLight::new(
                light.position.into(),
                light.radius,
                light.color.into(),
                light.intensity,
            )
        })
        .collect()
}

impl Scene {
    pub fn from_wavefront(image_directory: &Path, obj: &obj::Obj, mtl: &mtl::Mtl) -> Scene {
        let materials = materials_from_mtl(image_directory, mtl);
        let (geometries, properties) = obj
            .chunks
            .iter()
            .flat_map(|chunk| {
                chunk.faces.iter().map(|face| {
                    assert!(
                        face.points.len() == 3,
                        "Only tringular faces supported but found {} vertices.",
                        face.points.len()
                    );
                    let geometry = Geometry::from(Triangle {
                        v0: obj.index_vertex(&face.points[0]).into(),
                        v1: obj.index_vertex(&face.points[1]).into(),
                        v2: obj.index_vertex(&face.points[2]).into(),
                    });
                    let normals = TriangleNormals {
                        n0: obj.index_normal(&face.points[0]).into(),
                        n1: obj.index_normal(&face.points[1]).into(),
                        n2: obj.index_normal(&face.points[2]).into(),
                    };
                    let texcoords = TriangleTexcoords {
                        uv0: obj.index_texcoord(&face.points[0]).into(),
                        uv1: obj.index_texcoord(&face.points[1]).into(),
                        uv2: obj.index_texcoord(&face.points[2]).into(),
                    };
                    let material_index = materials
                        .iter()
                        .enumerate()
                        .find(|m| m.1 .0 == &chunk.material)
                        .unwrap()
                        .0;
                    let properties = TriangleProperties {
                        normals,
                        texcoords,
                        material_index,
                    };
                    (geometry, properties)
                })
            })
            .unzip();
        Scene {
            geometries,
            properties,
            materials: materials.into_iter().map(|m| m.1).collect(),
            cameras: collect_cameras(mtl),
            lights: collect_lights(mtl),
            environment: Vec3::new(0.8, 0.8, 0.8),
        }
    }

    pub fn read_obj_file_with_print_logging(path: &Path) -> Scene {
        println!("Loading {}...", path.display());
        let obj = obj::obj(&mut BufReader::new(File::open(path).unwrap()));
        println!("  Chunks: {}", obj.chunks.len());
        println!("  Vertices: {}", obj.vertices.len());
        println!("  Normals: {}", obj.normals.len());
        println!("  Texcoords: {}", obj.texcoords.len());

        let mtl_path = path.parent().unwrap().join(&obj.mtl_lib);
        println!("Loading {}...", mtl_path.display());
        let mtl = mtl::mtl(&mut BufReader::new(File::open(&mtl_path).unwrap()));
        println!("  Materials: {}", mtl.materials.len());
        println!("  Lights: {}", mtl.lights.len());
        println!("  Cameras: {}", mtl.cameras.len());

        println!("Building scene...");
        let scene = Scene::from_wavefront(mtl_path.parent().unwrap(), &obj, &mtl);
        println!("  Geometries: {}", scene.geometries.len());
        scene
    }

    #[inline]
    pub fn get_material(&self, index: u32) -> &Material {
        &self.materials[self.properties[index as usize].material_index]
    }

    #[inline]
    pub fn get_normal(&self, index: u32, intersection: &RayIntersection) -> Vec3 {
        self.properties[index as usize]
            .normals
            .lerp(intersection.u, intersection.v)
    }

    #[inline]
    pub fn get_texcoord(&self, index: u32, intersection: &RayIntersection) -> Vec2 {
        self.properties[index as usize]
            .texcoords
            .lerp(intersection.u, intersection.v)
    }
}
