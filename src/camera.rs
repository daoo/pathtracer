use crate::geometry::ray::*;
use nalgebra::{Vector3, UnitVector3};

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Vector3<f32>,
    pub direction: UnitVector3<f32>,
    pub up: UnitVector3<f32>,
    pub right: UnitVector3<f32>,
    pub fov_degrees: f32,
}

impl Camera {
    pub fn new(position: &Vector3<f32>, target: &Vector3<f32>, up: &Vector3<f32>, fov_degrees: f32) -> Camera {
        let direction = UnitVector3::new_normalize(target - position);
        Camera {
            position: *position,
            direction,
            up: UnitVector3::new_normalize(*up),
            right: UnitVector3::new_normalize(direction.cross(up)),
            fov_degrees
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pinhole {
    pub position: Vector3<f32>,
    pub plane: Vector3<f32>,
    pub dx: Vector3<f32>,
    pub dy: Vector3<f32>,
}

impl Pinhole {
    pub fn new(camera: &Camera, aspect_ratio: f32) -> Pinhole {
        let half_fov_radians = camera.fov_degrees * std::f32::consts::PI / 360.0;
        let x = camera.right.into_inner() * (half_fov_radians.sin() * aspect_ratio);
        let y = camera.up.into_inner() * half_fov_radians.sin();
        let z = camera.direction.into_inner() * half_fov_radians.cos();

        Pinhole {
            position: camera.position,
            plane: z + y - x,
            dx: 2.0 * x,
            dy: -2.0 * y,
        }
    }

    pub fn ray(&self, x: f32, y: f32) -> Ray {
        Ray {
            origin: self.position,
            direction: self.plane + x * self.dx + y * self.dy,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use nalgebra::vector;

//     #[test]
//     fn ray_in_image_plane_center() {
//         let camera = Camera::new(
//             &vector![1.0, 0.0, 0.0],
//             &vector![-1.0, 0.0, 0.0],
//             &vector![0.0, 1.0, 0.0],
//             1.0,
//         );
//         let pinhole = Pinhole::new(&camera, 1.0);

//         let actual = pinhole.ray(0.5, 0.5);

//         let direction_error = (actual.direction - vector![-1.0, 0.0, 0.0]).norm();
//         assert_eq!(actual.origin, vector![1.0, 0.0, 0.0]);
//         assert!(direction_error <= 0.1, "direction_error = {}", direction_error);
//     }
// }
