use nalgebra::{Vector2, Vector3};

use crate::{aap::Aap, axis::Axis};

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

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    pub v0: Vector3<f32>,
    pub v1: Vector3<f32>,
    pub v2: Vector3<f32>,
}

impl Triangle {
    pub fn min(&self) -> Vector3<f32> {
        self.v0.inf(&self.v1.inf(&self.v2))
    }

    pub fn max(&self) -> Vector3<f32> {
        self.v0.sup(&self.v1.sup(&self.v2))
    }

    pub fn base0(&self) -> Vector3<f32> {
        self.v1 - self.v0
    }

    pub fn base1(&self) -> Vector3<f32> {
        self.v2 - self.v0
    }

    pub fn base_center(&self) -> Vector3<f32> {
        self.v0 + 0.5 * self.base0() + 0.5 * self.base1()
    }

    pub fn edge0(&self) -> Vector3<f32> {
        self.v1 - self.v0
    }

    pub fn edge1(&self) -> Vector3<f32> {
        self.v2 - self.v1
    }

    pub fn edge2(&self) -> Vector3<f32> {
        self.v0 - self.v2
    }

    pub fn param(&self, u: f32, v: f32) -> Vector3<f32> {
        assert!(u >= 0.0 && v >= 0.0 && u + v <= 1.0);
        self.v0 + u * self.base0() + v * self.base1()
    }

    pub fn from_arrays(arrays: [[f32; 3]; 3]) -> Triangle {
        let [v0, v1, v2] = arrays;
        Triangle {
            v0: v0.into(),
            v1: v1.into(),
            v2: v2.into(),
        }
    }

    pub fn as_arrays(&self) -> [[f32; 3]; 3] {
        [self.v0.into(), self.v1.into(), self.v2.into()]
    }

    pub fn as_axially_aligned(&self) -> Option<AxiallyAlignedTriangle> {
        let check_axis = |axis| {
            (self.v0[axis] == self.v1[axis] && self.v0[axis] == self.v2[axis]).then_some(
                AxiallyAlignedTriangle {
                    plane: Aap {
                        axis,
                        distance: self.v0[axis],
                    },
                    v0: axis.remove_from(self.v0),
                    v1: axis.remove_from(self.v1),
                    v2: axis.remove_from(self.v2),
                },
            )
        };

        check_axis(Axis::X)
            .or(check_axis(Axis::Y))
            .or(check_axis(Axis::Z))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max() {
        let triangle = Triangle {
            v0: Vector3::new(1., 2., 3.),
            v1: Vector3::new(4., 5., 6.),
            v2: Vector3::new(7., 8., 9.),
        };
        assert_eq!(triangle.min(), Vector3::new(1., 2., 3.));
        assert_eq!(triangle.max(), Vector3::new(7., 8., 9.));
    }

    #[test]
    fn test_center() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 1., 1.),
            v2: Vector3::new(-1., -1., -1.),
        };
        assert_eq!(triangle.base_center(), Vector3::new(0., 0., 0.));
    }
}
