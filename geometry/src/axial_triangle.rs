use nalgebra::Vector2;

use crate::aap::Aap;

#[derive(Clone, Debug, PartialEq)]
pub struct AxiallyAlignedTriangle {
    pub plane: Aap,
    pub v0: Vector2<f32>,
    pub v1: Vector2<f32>,
    pub v2: Vector2<f32>,
}

impl AxiallyAlignedTriangle {
    pub fn base0(&self) -> Vector2<f32> {
        self.v1 - self.v0
    }

    pub fn base1(&self) -> Vector2<f32> {
        self.v2 - self.v0
    }

    pub fn param(&self, u: f32, v: f32) -> Vector2<f32> {
        debug_assert!(u >= 0.0 && v >= 0.0 && u + v <= 1.0);
        self.v0 + u * self.base0() + v * self.base1()
    }
}
