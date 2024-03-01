use nalgebra::Vector2;
use nalgebra::Vector3;

pub struct Triangle {
  v0: Vector3<f32>,
  v1: Vector3<f32>,
  v2: Vector3<f32>,
  n0: Vector3<f32>,
  n1: Vector3<f32>,
  n2: Vector3<f32>,
  uv0: Vector2<f32>,
  uv1: Vector2<f32>,
  uv2: Vector2<f32>,

  // const void* tag;
}

impl Triangle {
    pub fn min(&self) -> Vector3<f32> {
        self.v0.inf(&self.v1.inf(&self.v2))
    }

    pub fn max(&self) -> Vector3<f32> {
        self.v0.sup(&self.v1.sup(&self.v2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max() {
        let triangle = Triangle {
            v0: Vector3::new(1.0, 2.0, 3.0),
            v1: Vector3::new(4.0, 5.0, 6.0),
            v2: Vector3::new(7.0, 8.0, 9.0),
            n0: Vector3::new(10.0, 11.0, 12.0),
            n1: Vector3::new(13.0, 14.0, 15.0),
            n2: Vector3::new(16.0, 17.0, 18.0),
            uv0: Vector2::new(19.0, 20.0),
            uv1: Vector2::new(21.0, 22.0),
            uv2: Vector2::new(23.0, 24.0),
        };
        assert_eq!(triangle.min(), Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(triangle.max(), Vector3::new(7.0, 8.0, 9.0));
    }
}
