use nalgebra::Vector3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn between(a: &Vector3<f32>, b: &Vector3<f32>) -> Ray {
        Ray { origin: a.clone(), direction: b - a, }
    }

    pub fn extend(&self, t: f32) -> Ray {
        Ray { origin: self.origin.clone(), direction: self.direction + t * self.direction }
    }

    pub fn param(&self, t: f32) -> Vector3<f32> {
        &self.origin + t * &self.direction
    }

    pub fn reverse(&self) -> Ray {
        Ray { origin: self.origin + self.direction, direction: -self.direction }
    }
}
