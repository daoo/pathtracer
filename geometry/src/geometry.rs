use glam::{Vec2, Vec3};
use wavefront::{mtl, obj};

use crate::{
    aabb::Aabb,
    axial_triangle::AxiallyAlignedTriangle,
    intersection::RayIntersection,
    ray::Ray,
    sphere::Sphere,
    triangle::{Triangle, TriangleNormals, TriangleTexcoords},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Geometry {
    Triangle(Triangle),
    AxiallyAlignedTriangle(AxiallyAlignedTriangle),
    Sphere(Sphere),
}

impl Geometry {
    #[inline]
    pub fn min(&self) -> Vec3 {
        match self {
            Geometry::Triangle(g) => g.min(),
            Geometry::AxiallyAlignedTriangle(g) => g.min(),
            Geometry::Sphere(s) => s.min(),
        }
    }

    #[inline]
    pub fn max(&self) -> Vec3 {
        match self {
            Geometry::Triangle(g) => g.max(),
            Geometry::AxiallyAlignedTriangle(g) => g.max(),
            Geometry::Sphere(s) => s.max(),
        }
    }

    #[inline]
    pub fn intersect_ray(&self, ray: &Ray) -> Option<RayIntersection> {
        match self {
            Geometry::Triangle(g) => g.intersect_ray(ray),
            Geometry::AxiallyAlignedTriangle(g) => g.intersect_ray(ray),
            Geometry::Sphere(s) => s.intersect_ray(ray),
        }
    }

    #[inline]
    pub fn clip_aabb(&self, aabb: &Aabb) -> Option<Aabb> {
        match self {
            Geometry::Triangle(g) => g.clip_aabb(aabb),
            Geometry::AxiallyAlignedTriangle(g) => g.clip_aabb(aabb),
            Geometry::Sphere(_) => todo!(),
        }
    }
}

impl From<Triangle> for Geometry {
    #[inline]
    fn from(value: Triangle) -> Self {
        value
            .as_axially_aligned()
            .map(Geometry::AxiallyAlignedTriangle)
            .unwrap_or(Geometry::Triangle(value))
    }
}

impl From<Sphere> for Geometry {
    #[inline]
    fn from(value: Sphere) -> Self {
        Geometry::Sphere(value)
    }
}

pub enum GeometryProperties<M> {
    Triangle {
        normals: TriangleNormals,
        texcoords: TriangleTexcoords,
        material: M,
    },
    AxiallyAlignedTriangle {
        normals: TriangleNormals,
        texcoords: TriangleTexcoords,
        material: M,
    },
    Sphere {
        material: M,
    },
}

impl<M> GeometryProperties<M> {
    #[inline]
    pub fn material(&self) -> &M {
        match self {
            GeometryProperties::Triangle {
                normals: _,
                texcoords: _,
                material,
            } => material,
            GeometryProperties::AxiallyAlignedTriangle {
                normals: _,
                texcoords: _,
                material,
            } => material,
            GeometryProperties::Sphere { material } => material,
        }
    }

    #[inline]
    pub fn compute_normal(&self, u: f32, v: f32) -> Vec3 {
        match self {
            GeometryProperties::Triangle {
                normals,
                texcoords: _,
                material: _,
            } => normals.lerp(u, v),
            GeometryProperties::AxiallyAlignedTriangle {
                normals,
                texcoords: _,
                material: _,
            } => normals.lerp(u, v),
            GeometryProperties::Sphere { material: _ } => {
                Vec3::new(u.sin() * v.cos(), u.sin() * v.sin(), u.cos())
            }
        }
    }

    #[inline]
    pub fn compute_texcoord(&self, u: f32, v: f32) -> Vec2 {
        match self {
            GeometryProperties::Triangle {
                normals: _,
                texcoords,
                material: _,
            } => texcoords.lerp(u, v),
            GeometryProperties::AxiallyAlignedTriangle {
                normals: _,
                texcoords,
                material: _,
            } => texcoords.lerp(u, v),
            GeometryProperties::Sphere { material: _ } => Vec2::new(u, v),
        }
    }
}

pub fn from_wavefront(
    obj: &obj::Obj,
    mtl: &mtl::Mtl,
) -> (Vec<Geometry>, Vec<GeometryProperties<usize>>) {
    let materials: Vec<&str> = mtl.materials.iter().map(|m| m.name.as_str()).collect();
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
                let material_index = materials.iter().position(|m| *m == chunk.material).unwrap();
                let properties = GeometryProperties::<usize>::Triangle {
                    normals,
                    texcoords,
                    material: material_index,
                };
                (geometry, properties)
            })
        })
        .unzip();
    (geometries, properties)
}
