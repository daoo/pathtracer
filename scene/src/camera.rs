use geometry::ray::Ray;
use glam::{UVec2, Vec2, Vec3};

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub fov_degrees: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, up: Vec3, fov_degrees: f32) -> Camera {
        let direction = (target - position).normalize();
        Camera {
            position,
            direction,
            up: up.normalize(),
            right: direction.cross(up).normalize(),
            fov_degrees,
        }
    }

    pub fn with_position(&self, position: Vec3) -> Self {
        Camera {
            position,
            direction: self.direction,
            up: self.up,
            right: self.right,
            fov_degrees: self.fov_degrees,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pinhole {
    pub camera: Camera,
    pub size: UVec2,
    pub plane: Vec3,
    pub dx: Vec3,
    pub dy: Vec3,
}

impl Pinhole {
    pub fn new(camera: Camera, size: UVec2) -> Pinhole {
        let aspect_ratio = size.x as f32 / size.y as f32;
        let half_fov_radians = camera.fov_degrees * std::f32::consts::PI / 360.0;
        let x = camera.right * (half_fov_radians.sin() * aspect_ratio);
        let y = camera.up * half_fov_radians.sin();
        let z = camera.direction * half_fov_radians.cos();

        Pinhole {
            camera,
            size,
            plane: z + y - x,
            dx: 2.0 * x,
            dy: -2.0 * y,
        }
    }

    #[inline]
    pub fn ray(&self, v: Vec2) -> Ray {
        Ray::new(
            self.camera.position,
            self.plane + v.x * self.dx + v.y * self.dy,
        )
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
