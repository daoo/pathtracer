use nalgebra::{vector, Vector3};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Axis {
    pub fn from_u32(n: u32) -> Axis {
        match n % 3 {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => panic!("Impossible modulo result."),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Aap {
    pub axis: Axis,
    pub distance: f32,
}

impl Aap {
    pub fn vector(&self) -> Vector3<f32> {
        match self.axis {
            Axis::X => vector![self.distance, 0.0, 0.0],
            Axis::Y => vector![0.0, self.distance, 0.0],
            Axis::Z => vector![0.0, 0.0, self.distance],
        }
    }
}

impl std::ops::Index<Axis> for Vector3<f32> {
    type Output = f32;
    fn index(&self, idx: Axis) -> &f32 {
        &self[idx as usize]
    }
}

impl std::ops::IndexMut<Axis> for Vector3<f32> {
    fn index_mut(&mut self, idx: Axis) -> &mut f32 {
        &mut self[idx as usize]
    }
}
