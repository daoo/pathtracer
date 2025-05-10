use std::ops::RangeInclusive;

use glam::{Vec2, Vec3};
use wavefront::{mtl, obj};

use crate::{
    ray::Ray,
    shape::{Shape, ShapeIntersection},
    triangle::{Triangle, TriangleNormals, TriangleTexcoords},
};

#[derive(Clone, Debug, PartialEq)]
pub enum GeometryProperties {
    Triangle {
        material: usize,
        normals: TriangleNormals,
        texcoords: TriangleTexcoords,
    },
    AxiallyAlignedTriangle {
        material: usize,
        normals: TriangleNormals,
        texcoords: TriangleTexcoords,
    },
    Sphere {
        material: usize,
        radius: f32,
    },
}

impl GeometryProperties {
    #[inline]
    pub fn material(&self) -> usize {
        match self {
            GeometryProperties::Triangle {
                normals: _,
                texcoords: _,
                material,
            } => *material,
            GeometryProperties::AxiallyAlignedTriangle {
                normals: _,
                texcoords: _,
                material,
            } => *material,
            GeometryProperties::Sphere {
                material,
                radius: _,
            } => *material,
        }
    }

    #[inline]
    pub fn compute_normal(&self, intersection: &ShapeIntersection) -> Vec3 {
        match self {
            GeometryProperties::Triangle {
                normals,
                texcoords: _,
                material: _,
            } => normals.lerp(intersection.u().unwrap(), intersection.v().unwrap()),
            GeometryProperties::AxiallyAlignedTriangle {
                normals,
                texcoords: _,
                material: _,
            } => normals.lerp(intersection.u().unwrap(), intersection.v().unwrap()),
            GeometryProperties::Sphere {
                material: _,
                radius: _,
            } => intersection.normal().unwrap(),
        }
    }

    #[inline]
    pub fn compute_texcoord(&self, intersection: &ShapeIntersection) -> Vec2 {
        match self {
            GeometryProperties::Triangle {
                normals: _,
                texcoords,
                material: _,
            } => texcoords.lerp(intersection.u().unwrap(), intersection.v().unwrap()),
            GeometryProperties::AxiallyAlignedTriangle {
                normals: _,
                texcoords,
                material: _,
            } => texcoords.lerp(intersection.u().unwrap(), intersection.v().unwrap()),
            GeometryProperties::Sphere {
                material: _,
                radius,
            } => {
                let normal = intersection.normal().unwrap();
                let theta = normal.y.atan2(normal.x);
                let phi = (normal.z / radius).acos();
                Vec2::new(theta, phi)
            }
        }
    }
}

pub fn from_wavefront(obj: &obj::Obj, mtl: &mtl::Mtl) -> (Vec<Shape>, Vec<GeometryProperties>) {
    let materials: Vec<&str> = mtl.materials.iter().map(|m| m.name.as_str()).collect();
    let (shapes, properties) = obj
        .chunks
        .iter()
        .flat_map(|chunk| {
            chunk.faces.iter().map(|face| {
                assert!(
                    face.points.len() == 3,
                    "Only tringular faces supported but found {} vertices.",
                    face.points.len()
                );
                let shape = Shape::from(Triangle {
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
                let properties = GeometryProperties::Triangle {
                    normals,
                    texcoords,
                    material: material_index,
                };
                (shape, properties)
            })
        })
        .unzip();
    (shapes, properties)
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeometryIntersection {
    pub index: u32,
    pub inner: ShapeIntersection,
}

impl GeometryIntersection {
    #[inline]
    pub fn new(index: u32, inner: ShapeIntersection) -> Self {
        GeometryIntersection { index, inner }
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        if self.inner.t() <= other.inner.t() {
            self
        } else {
            other
        }
    }

    #[inline]
    pub fn point(&self, ray: &Ray) -> Vec3 {
        self.inner.point(ray)
    }

    #[inline]
    pub fn ray(&self, ray: &Ray) -> Ray {
        self.inner.ray(ray)
    }
}

pub fn intersect_closest_geometry(
    shapes: &[Shape],
    indices: impl Iterator<Item = u32>,
    ray: &Ray,
    t_range: RangeInclusive<f32>,
) -> Option<GeometryIntersection> {
    indices
        .filter_map(|index| {
            let geometry = unsafe { shapes.get_unchecked(index as usize) };
            geometry.intersect_ray(ray).and_then(|intersection| {
                t_range
                    .contains(&intersection.t())
                    .then_some(GeometryIntersection::new(index, intersection))
            })
        })
        .reduce(GeometryIntersection::min)
}

#[cfg(test)]
mod tests {
    use crate::sphere::Sphere;

    use super::*;

    #[test]
    fn compute_normal_origo_sphere_intersected_along_x_axis() {
        let sphere = Sphere {
            center: Vec3::ZERO,
            radius: 1.0,
        };
        let properties = GeometryProperties::Sphere {
            material: 0,
            radius: sphere.radius,
        };
        let ray = Ray::between(Vec3::new(2.0, 0.0, 0.0), Vec3::new(-2.0, 0.0, 0.0));
        let intersection = ShapeIntersection::Sphere(sphere.intersect_ray(&ray).unwrap());

        let actual = properties.compute_normal(&intersection);

        assert_eq!(actual, Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn compute_normal_origo_sphere_intersected_along_y_axis() {
        let sphere = Sphere {
            center: Vec3::ZERO,
            radius: 1.0,
        };
        let properties = GeometryProperties::Sphere {
            material: 0,
            radius: sphere.radius,
        };
        let ray = Ray::between(Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, -2.0, 0.0));
        let intersection = ShapeIntersection::Sphere(sphere.intersect_ray(&ray).unwrap());

        let actual = properties.compute_normal(&intersection);

        assert_eq!(actual, Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn compute_normal_origo_sphere_intersected_along_z_axis() {
        let sphere = Sphere {
            center: Vec3::ZERO,
            radius: 1.0,
        };
        let properties = GeometryProperties::Sphere {
            material: 0,
            radius: sphere.radius,
        };
        let ray = Ray::between(Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -2.0));
        let intersection = ShapeIntersection::Sphere(sphere.intersect_ray(&ray).unwrap());

        let actual = properties.compute_normal(&intersection);

        assert_eq!(actual, Vec3::new(0.0, 0.0, 1.0));
    }
}
