use nalgebra::Vector3;
use nalgebra;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub struct Aabb {
    pub center: Vector3<f32>,
    pub half_size: Vector3<f32>,
}

impl Aabb {
    pub fn from_extents(min: &Vector3<f32>, max: &Vector3<f32>) -> Aabb {
        let size = max - min;
        let half_size = size / 2.0;
        Aabb { center: min + half_size, half_size }
    }

    pub fn unit() -> Aabb {
        Aabb {
            center: Vector3::new(0.0, 0.0, 0.0),
            half_size: Vector3::new(0.5, 0.5, 0.5),
        }
    }

    pub fn min(&self) -> Vector3<f32> {
        &self.center - &self.half_size
    }

    pub fn max(&self) -> Vector3<f32> {
        &self.center + &self.half_size
    }

    pub fn surface_area(&self) -> f32 {
        8.0 * (&self.half_size.x * &self.half_size.y + &self.half_size.x * &self.half_size.z + &self.half_size.y * &self.half_size.z)
    }

    pub fn volume(&self) -> f32 {
        8.0 * &self.half_size.x * &self.half_size.y * &self.half_size.z
    }

    pub fn translate(&self, delta: &Vector3<f32>) -> Aabb {
        Aabb{center: self.center + delta, half_size: self.half_size}
    }

    pub fn enlarge(&self, delta: &Vector3<f32>) -> Aabb {
        Aabb{center: self.center, half_size: &self.half_size + delta}
    }

    pub fn clamp(&self, v: Vector3<f32>) -> Vector3<f32> {
        nalgebra::clamp(v, self.min(), self.max())
    }
}
