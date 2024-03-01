use nalgebra::Vector3;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Axis { X = 0, Y = 1, Z = 2 }

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub struct Aap {
    pub axis: Axis,
    pub distance: f32,
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
