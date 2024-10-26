use geometry::{
    geometry::{Geometry, GeometryProperties},
    intersection::GeometryIntersection,
    triangle::{Triangle, TriangleNormals, TriangleTexcoords},
};
use glam::{Vec2, Vec3};
use wavefront::{mtl, obj};

pub struct Scene {
    geometries: Vec<Geometry>,
    properties: Vec<GeometryProperties<usize>>,
    environment: Vec3,
}

pub struct SceneIntersection {
    pub inner: GeometryIntersection,
    pub normal: Vec3,
    pub texcoord: Vec2,
}

impl Scene {
    pub fn from_wavefront(obj: &obj::Obj, mtl: &mtl::Mtl) -> Scene {
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
                    let material_index =
                        materials.iter().position(|m| *m == chunk.material).unwrap();
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
            environment: Vec3::new(0.8, 0.8, 0.8),
        }
    }

    pub fn build_with_print_logging(obj: &obj::Obj, mtl: &mtl::Mtl) -> Scene {
        println!("Building scene...");
        let scene = Scene::from_wavefront(obj, mtl);
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
    pub fn lookup_intersection(&self, inner: &GeometryIntersection) -> &GeometryProperties<usize> {
        &self.properties[inner.index as usize]
    }
}
