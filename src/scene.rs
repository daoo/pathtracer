use crate::camera::*;
use crate::geometry::algorithms::*;
use crate::geometry::ray::*;
use crate::geometry::triangle::*;
use crate::light::*;
use crate::material::*;
use crate::wavefront::*;
use std::rc::Rc;

pub struct Scene {
    pub triangles: Vec<Triangle>,
    pub materials: Vec<Rc<dyn Material>>,
    pub cameras: Vec<Camera>,
    pub lights: Vec<SphericalLight>,
}

fn triangles_from_obj(obj: &obj::Obj) -> Vec<Triangle> {
    obj.chunks
        .iter()
        .map(|chunk| chunk.faces
             .iter()
             .map(|face| Triangle {
                v0: obj.index_vertex(&face.p0),
                v1: obj.index_vertex(&face.p1),
                v2: obj.index_vertex(&face.p2),
             }))
        .flatten()
        .collect()
}

fn blend1_from_mtl(material: &mtl::Material) -> Rc<dyn Material> {
    let refraction = SpecularRefractiveMaterial { index_of_refraction: material.index_of_refraction };
    let reflection = DiffuseReflectiveMaterial { reflectance: material.diffuse_reflection };
    if material.transparency >= 0.99 {
        Rc::new(refraction)
    } else if material.transparency <= 0.01 {
        Rc::new(reflection)
    } else {
        Rc::new(BlendMaterial {
            first: Rc::new(refraction),
            second: Rc::new(reflection),
            factor: material.transparency
        })
    }
}

fn blend0_from_mtl(material: &mtl::Material, blend1: Rc<dyn Material>) -> Rc<dyn Material> {
    let specular = SpecularReflectiveMaterial { reflectance: material.specular_reflection };
    if material.reflection_90_degrees >= 0.99 {
        Rc::new(FresnelBlendMaterial {
            reflection: Rc::new(specular),
            refraction: blend1,
            r0: material.reflection_0_degrees,
        })
    } else if material.reflection_90_degrees <= 0.01 {
        blend1
    } else {
        Rc::new(BlendMaterial {
            first: Rc::new(FresnelBlendMaterial {
                reflection: Rc::new(specular),
                refraction: blend1.clone(),
                r0: material.reflection_0_degrees,
            }),
            second: blend1,
            factor: material.reflection_90_degrees
        })
    }
}

fn material_from_mtl(material: &mtl::Material) -> Rc<dyn Material> {
    blend0_from_mtl(&material, blend1_from_mtl(&material))
}

fn materials_from_mtl(mtl: &mtl::Mtl) -> Vec<Rc<dyn Material>> {
    mtl.materials.iter().map(material_from_mtl).collect()
}

fn cameras_from_mtl(mtl: &mtl::Mtl) -> Vec<Camera> {
    mtl.cameras
        .iter()
        .map(|camera| Camera::new(&camera.position, &camera.target, &camera.up, camera.fov))
        .collect()
}

fn lights_from_mtl(mtl: &mtl::Mtl) -> Vec<SphericalLight> {
    mtl.lights
        .iter()
        .map(|light| SphericalLight::new(light.position, light.radius, &light.color, light.intensity))
        .collect()
}

impl Scene {
    pub fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<TriangleRayIntersection> {
        let triangle_refs: Vec<&Triangle> = self.triangles.iter().map(|t| t).collect::<Vec<_>>();
        intersect_closest_triangle_ray(&triangle_refs, ray, tmin, tmax)
    }

    pub fn intersect_any(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        self.intersect(ray, tmin, tmax).is_some()
    }

    pub fn from_wavefront(obj: &obj::Obj, mtl: &mtl::Mtl) -> Scene {
        Scene {
            triangles: triangles_from_obj(obj),
            materials: materials_from_mtl(mtl),
            cameras: cameras_from_mtl(mtl),
            lights: lights_from_mtl(mtl),
        }
    }
}
