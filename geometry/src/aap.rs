use nalgebra::Vector3;

use super::axis::Axis;

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
