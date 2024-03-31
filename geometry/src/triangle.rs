use nalgebra::Vector3;

use super::ray::Ray;

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

    pub fn edge0_ray(&self) -> Ray {
        Ray {
            origin: self.v0,
            direction: self.edge0(),
        }
    }

    pub fn edge1_ray(&self) -> Ray {
        Ray {
            origin: self.v1,
            direction: self.edge1(),
        }
    }

    pub fn edge2_ray(&self) -> Ray {
        Ray {
            origin: self.v2,
            direction: self.edge2(),
        }
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
