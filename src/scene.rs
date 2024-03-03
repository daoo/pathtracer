use crate::camera::*;
use crate::geometry::algorithms::*;
use crate::geometry::ray::*;
use crate::geometry::triangle::*;
use crate::light::*;
use crate::material::*;

pub struct Scene {
    pub triangles: Vec<Triangle>,
    pub materials: Vec<Box<dyn Material>>,
    pub cameras: Vec<Camera>,
    pub lights: Vec<SphericalLight>,
}

impl Scene {
    pub fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<TriangleRayIntersection> {
        let triangle_refs: Vec<&Triangle> = self.triangles.iter().map(|t| t).collect::<Vec<_>>();
        intersect_closest_triangle_ray(&triangle_refs, ray, tmin, tmax)
    }

    pub fn intersect_any(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        self.intersect(ray, tmin, tmax).is_some()
    }
}
