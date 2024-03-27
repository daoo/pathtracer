use nalgebra::Vector3;

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
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Aap {
    pub axis: Axis,
    pub distance: f32,
}

impl Aap {
    pub fn new_x(distance: f32) -> Aap {
        Aap {
            axis: Axis::X,
            distance,
        }
    }

    pub fn new_y(distance: f32) -> Aap {
        Aap {
            axis: Axis::Y,
            distance,
        }
    }

    pub fn new_z(distance: f32) -> Aap {
        Aap {
            axis: Axis::Z,
            distance,
        }
    }

    pub fn vector(&self) -> Vector3<f32> {
        self.axis.as_vector3(self.distance)
    }

    pub fn total_cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.axis
            .cmp(&other.axis)
            .then(f32::total_cmp(&self.distance, &other.distance))
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
