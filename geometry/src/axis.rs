use nalgebra::{Vector2, Vector3};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn as_vector3(&self, v: f32) -> Vector3<f32> {
        match self {
            Axis::X => Vector3::new(v, 0.0, 0.0),
            Axis::Y => Vector3::new(0.0, v, 0.0),
            Axis::Z => Vector3::new(0.0, 0.0, v),
        }
    }

    pub fn add_to(&self, v: Vector2<f32>, s: f32) -> Vector3<f32> {
        match self {
            Axis::X => Vector3::new(s, v.x, v.y),
            Axis::Y => Vector3::new(v.x, s, v.y),
            Axis::Z => Vector3::new(v.x, v.y, s),
        }
    }

    pub fn remove_from(&self, v: Vector3<f32>) -> Vector2<f32> {
        match self {
            Axis::X => v.yz(),
            Axis::Y => v.xz(),
            Axis::Z => v.xy(),
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
