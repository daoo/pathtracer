use crate::geometry::ray::*;
use nalgebra::{Vector3, UnitVector3};

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Vector3<f32>,
    pub direction: UnitVector3<f32>,
    pub up: UnitVector3<f32>,
    pub right: UnitVector3<f32>,
    pub fov: f32,
}

impl Camera {
    pub fn new(position: &Vector3<f32>, target: &Vector3<f32>, up: &Vector3<f32>, fov: f32) -> Camera {
        let direction = UnitVector3::new_normalize(target - position);
        Camera {
            position: position.clone(),
            direction,
            up: UnitVector3::new_normalize(*up),
            right: UnitVector3::new_normalize(direction.cross(up)),
            fov
        }
    }
}

struct Pinhole {
    pub position: Vector3<f32>,
    pub origin: Vector3<f32>,
    pub dx: Vector3<f32>,
    pub dy: Vector3<f32>,
}

impl Pinhole {
    pub fn new(camera: &Camera, aspect_ratio: f32) -> Pinhole {
        let fov_half = camera.fov / 2.0;
        let x = camera.up.into_inner() * fov_half.sin();
        let y = camera.right.into_inner() * fov_half.sin() * aspect_ratio;
        let z = camera.direction.into_inner() * fov_half.cos();

        Pinhole {
            position: camera.position,
            origin: z - y - x,
            dx: 2.0 * y,
            dy: 2.0 * x,
        }
    }

    pub fn ray(&self, x: f32, y: f32) -> Ray {
        Ray {
            origin: self.position,
            direction: self.origin + x * self.dx + y * self.dy,
        }
    }
}
