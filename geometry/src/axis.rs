use glam::{Vec2, Vec3, Vec3Swizzles};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Axis {
    #[inline]
    pub fn from_u32(n: u32) -> Axis {
        match n % 3 {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => panic!("Impossible modulo result."),
        }
    }

    pub fn as_vector3(&self, v: f32) -> Vec3 {
        match self {
            Axis::X => Vec3::new(v, 0.0, 0.0),
            Axis::Y => Vec3::new(0.0, v, 0.0),
            Axis::Z => Vec3::new(0.0, 0.0, v),
        }
    }

    pub fn add_to(&self, v: Vec2, s: f32) -> Vec3 {
        match self {
            Axis::X => Vec3::new(s, v.x, v.y),
            Axis::Y => Vec3::new(v.x, s, v.y),
            Axis::Z => Vec3::new(v.x, v.y, s),
        }
    }

    pub fn remove_from(&self, v: Vec3) -> Vec2 {
        match self {
            Axis::X => v.yz(),
            Axis::Y => v.xz(),
            Axis::Z => v.xy(),
        }
    }
}

impl std::ops::Index<Axis> for Vec3 {
    type Output = f32;
    #[inline]
    fn index(&self, idx: Axis) -> &f32 {
        &self[idx as usize]
    }
}

impl std::ops::IndexMut<Axis> for Vec3 {
    #[inline]
    fn index_mut(&mut self, idx: Axis) -> &mut f32 {
        &mut self[idx as usize]
    }
}
