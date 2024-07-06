use glam::Vec3;

#[derive(Clone, Debug)]
pub struct SphericalLight {
    pub center: Vec3,
    pub intensity: Vec3,
    pub radius: f32,
}

impl SphericalLight {
    pub fn new(center: Vec3, radius: f32, color: Vec3, intensity: f32) -> SphericalLight {
        SphericalLight {
            center,
            intensity: color * intensity,
            radius,
        }
    }

    pub fn emitted(&self, point: Vec3) -> Vec3 {
        self.intensity / (self.center - point).length_squared()
    }
}
