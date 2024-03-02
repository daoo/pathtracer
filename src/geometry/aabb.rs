use crate::geometry::aap::Aap;
use nalgebra::{vector, Vector3};
use nalgebra;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

    pub fn empty() -> Aabb {
        Aabb {
            center: Vector3::zeros(),
            half_size: Vector3::zeros(),
        }
    }

    pub fn unit() -> Aabb {
        Aabb {
            center: vector![0.0, 0.0, 0.0],
            half_size: vector![0.5, 0.5, 0.5],
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

    pub fn split(&self, plane: &Aap) -> (Aabb, Aabb) {
        let fst_half_axis = (plane.distance - self.min()[plane.axis]) / 2.0;
        let snd_half_axis = (self.max()[plane.axis] - plane.distance) / 2.0;
        assert!(fst_half_axis >= 0.0 && snd_half_axis >= 0.0);

        let mut fst_center = self.center;
        let mut fst_half_size = self.half_size;
        fst_center[plane.axis] = plane.distance - fst_half_axis;
        fst_half_size[plane.axis] = fst_half_axis;

        let mut snd_center = self.center;
        let mut snd_half_size = self.half_size;
        snd_center[plane.axis] = plane.distance + snd_half_axis;
        snd_half_size[plane.axis] = snd_half_axis;

        let fst = Aabb {center: fst_center, half_size: fst_half_size};
        let snd = Aabb {center: snd_center, half_size: snd_half_size};
        (fst, snd)
    }
}
