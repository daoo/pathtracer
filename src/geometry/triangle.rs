use nalgebra::Vector3;

#[derive(Debug, PartialEq)]
pub struct Triangle {
  pub v0: Vector3<f32>,
  pub v1: Vector3<f32>,
  pub v2: Vector3<f32>,
  // pub n0: Vector3<f32>,
  // pub n1: Vector3<f32>,
  // pub n2: Vector3<f32>,
  // pub uv0: Vector2<f32>,
  // pub uv1: Vector2<f32>,
  // pub uv2: Vector2<f32>,
}

impl Triangle {
    pub fn min(&self) -> Vector3<f32> {
        self.v0.inf(&self.v1.inf(&self.v2))
    }

    pub fn max(&self) -> Vector3<f32> {
        self.v0.sup(&self.v1.sup(&self.v2))
    }

    pub fn base0(&self) -> Vector3<f32> {
        &self.v1 - &self.v0
    }

    pub fn base1(&self) -> Vector3<f32> {
        &self.v2 - &self.v0
    }

    pub fn base_center(&self) -> Vector3<f32> {
        self.v0 + 0.5 * self.base0() + 0.5 * self.base1()
    }

    pub fn edge0(&self) -> Vector3<f32> {
        &self.v1 - &self.v0
    }

    pub fn edge1(&self) -> Vector3<f32> {
        &self.v2 - &self.v1
    }

    pub fn edge2(&self) -> Vector3<f32> {
        &self.v0 - &self.v2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::vector;

    #[test]
    fn test_min_max() {
        let triangle = Triangle {
            v0: vector![1., 2., 3.],
            v1: vector![4., 5., 6.],
            v2: vector![7., 8., 9.],
        };
        assert_eq!(triangle.min(), vector![1., 2., 3.]);
        assert_eq!(triangle.max(), vector![7., 8., 9.]);
    }

    #[test]
    fn test_center() {
        let triangle = Triangle{
            v0: vector![0., 0., 0.],
            v1: vector![1., 1., 1.],
            v2: vector![-1., -1., -1.],
        };
        assert_eq!(triangle.base_center(), vector![0., 0., 0.]);
    }
}
