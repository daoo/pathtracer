use nalgebra::Vector3;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn from(a: &Vector3<f32>, b: &Vector3<f32>) -> Ray {
        Ray { origin: a.clone(), direction: b - a, }
    }

    pub fn param(&self, t: f32) -> Vector3<f32> {
        &self.origin + t * &self.direction
    }
}
