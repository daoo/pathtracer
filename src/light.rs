use nalgebra::Vector3;

pub struct SphericalLight {
    pub center: Vector3<f32>,
    pub intensity: Vector3<f32>,
    pub radius: f32,
}

impl SphericalLight {
    pub fn from_color(center: Vector3<f32>, radius: f32, color: &Vector3<f32>, intensity: f32) -> SphericalLight {
        SphericalLight {
            center,
            intensity: color * intensity,
            radius
        }
    }
}
