use nalgebra::Vector3;

#[derive(Clone, Debug)]
pub struct SphericalLight {
    pub center: Vector3<f32>,
    pub intensity: Vector3<f32>,
    pub radius: f32,
}

impl SphericalLight {
    pub fn new(
        center: Vector3<f32>,
        radius: f32,
        color: &Vector3<f32>,
        intensity: f32,
    ) -> SphericalLight {
        SphericalLight {
            center,
            intensity: color * intensity,
            radius,
        }
    }

    pub fn emitted(&self, point: Vector3<f32>) -> Vector3<f32> {
        self.intensity / (self.center - point).norm_squared()
    }
}
