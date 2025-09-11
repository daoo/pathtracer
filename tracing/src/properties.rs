use geometry::{
    any_triangle::AnyTriangle,
    sphere::SphereIntersection,
    triangle::{Triangle, TriangleIntersection, TriangleNormals, TriangleTexcoords},
};
use glam::{Vec2, Vec3};
use wavefront::{mtl, obj};

#[derive(Clone, Debug, PartialEq)]
pub struct TriangleProperties {
    pub material: usize,
    pub normals: TriangleNormals,
    pub texcoords: TriangleTexcoords,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SphereProperties {
    pub material: usize,
    pub radius: f32,
}

impl TriangleProperties {
    #[inline]
    pub fn compute_normal(&self, intersection: &TriangleIntersection) -> Vec3 {
        self.normals.lerp(intersection.u, intersection.v)
    }

    #[inline]
    pub fn compute_texcoord(&self, intersection: &TriangleIntersection) -> Vec2 {
        self.texcoords.lerp(intersection.u, intersection.v)
    }
}

impl SphereProperties {
    pub fn compute_normal(&self, intersection: &SphereIntersection) -> Vec3 {
        intersection.normal
    }

    pub fn compute_texcoord(&self, intersection: &SphereIntersection) -> Vec2 {
        let normal = intersection.normal;
        let theta = normal.y.atan2(normal.x);
        let phi = (normal.z / self.radius).acos();
        Vec2::new(theta, phi)
    }
}

pub fn from_wavefront(
    obj: &obj::Obj,
    mtl: &mtl::Mtl,
) -> (Vec<AnyTriangle>, Vec<TriangleProperties>) {
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
                let triangle = AnyTriangle::from(Triangle {
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
                let properties = TriangleProperties {
                    normals,
                    texcoords,
                    material: material_index,
                };
                (triangle, properties)
            })
        })
        .unzip();
    (shapes, properties)
}

#[cfg(test)]
mod tests {
    use super::*;
    use geometry::ray::Ray;
    use geometry::sphere::Sphere;

    #[test]
    fn compute_normal_origo_sphere_intersected_along_x_axis() {
        let sphere = Sphere {
            center: Vec3::ZERO,
            radius: 1.0,
        };
        let properties = SphereProperties {
            material: 0,
            radius: sphere.radius,
        };
        let ray = Ray::between(Vec3::new(2.0, 0.0, 0.0), Vec3::new(-2.0, 0.0, 0.0));
        let intersection = sphere.intersect_ray(&ray).unwrap();

        let actual = properties.compute_normal(&intersection);

        assert_eq!(actual, Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn compute_normal_origo_sphere_intersected_along_y_axis() {
        let sphere = Sphere {
            center: Vec3::ZERO,
            radius: 1.0,
        };
        let properties = SphereProperties {
            material: 0,
            radius: sphere.radius,
        };
        let ray = Ray::between(Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, -2.0, 0.0));
        let intersection = sphere.intersect_ray(&ray).unwrap();

        let actual = properties.compute_normal(&intersection);

        assert_eq!(actual, Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn compute_normal_origo_sphere_intersected_along_z_axis() {
        let sphere = Sphere {
            center: Vec3::ZERO,
            radius: 1.0,
        };
        let properties = SphereProperties {
            material: 0,
            radius: sphere.radius,
        };
        let ray = Ray::between(Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -2.0));
        let intersection = sphere.intersect_ray(&ray).unwrap();

        let actual = properties.compute_normal(&intersection);

        assert_eq!(actual, Vec3::new(0.0, 0.0, 1.0));
    }
}
