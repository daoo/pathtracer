use geometry::{
    geometry::{Geometry, GeometryProperties},
    intersection::GeometryIntersection,
    triangle::{Triangle, TriangleNormals, TriangleTexcoords},
};
use glam::{Vec2, Vec3};
use material::Material;
use std::path::Path;
use wavefront::{mtl, obj};

pub mod material;

pub struct Scene {
    geometries: Vec<Geometry>,
    properties: Vec<GeometryProperties<usize>>,
    materials: Vec<Material>,
    environment: Vec3,
}

fn material_from_mtl(image_directory: &Path, material: &mtl::Material) -> Material {
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

fn materials_from_mtl<'m>(image_directory: &Path, mtl: &'m mtl::Mtl) -> Vec<(&'m str, Material)> {
    mtl.materials
        .iter()
        .map(|m| (m.name.as_str(), material_from_mtl(image_directory, m)))
        .collect()
}

pub struct SceneIntersection<'a> {
    pub inner: GeometryIntersection,
    pub material: &'a Material,
    pub normal: Vec3,
    pub texcoord: Vec2,
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
                        .position(|m| m.0 == chunk.material)
                        .unwrap();
                    let properties = GeometryProperties::<usize>::Triangle {
                        normals,
                        texcoords,
                        material: material_index,
                    };
                    (geometry, properties)
                })
            })
            .unzip();
        Scene {
            geometries,
            properties,
            materials: materials.into_iter().map(|m| m.1).collect(),
            environment: Vec3::new(0.8, 0.8, 0.8),
        }
    }

    pub fn build_with_print_logging(obj: &obj::Obj, mtl: &mtl::Mtl, mtl_path: &Path) -> Scene {
        println!("Building scene...");
        let scene = Scene::from_wavefront(mtl_path.parent().unwrap(), obj, mtl);
        println!("  Geometries: {}", scene.geometries.len());
        scene
    }

    #[inline]
    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }

    #[inline]
    pub fn environment(&self) -> Vec3 {
        self.environment
    }

    #[inline]
    pub fn lookup_intersection(&self, inner: GeometryIntersection) -> SceneIntersection<'_> {
        let properties = &self.properties[inner.index as usize];
        let normal = properties.compute_normal(inner.inner.u, inner.inner.v);
        let texcoord = properties.compute_texcoord(inner.inner.u, inner.inner.v);
        let material = &self.materials[*properties.material()];
        SceneIntersection {
            inner,
            material,
            normal,
            texcoord,
        }
    }
}
