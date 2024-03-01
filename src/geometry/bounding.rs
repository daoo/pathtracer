use crate::geometry::aabb::Aabb;
use crate::geometry::triangle::Triangle;

pub fn bounding(triangles: &[Triangle]) -> Aabb {
    if triangles.is_empty() {
        return Aabb::empty()
    }
    let mut a = triangles[0].min();
    let mut b = triangles[0].max();
    for triangle in triangles {
        a = a.inf(&triangle.min());
        b = b.sup(&triangle.max());
    }
    Aabb::from_extents(&a, &b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector2;
    use nalgebra::Vector3;

    fn triangle(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>) -> Triangle {
        Triangle {
            v0,
            v1,
            v2,
            n0: Vector3::zeros(),
            n1: Vector3::zeros(),
            n2: Vector3::zeros(),
            uv0: Vector2::zeros(),
            uv1: Vector2::zeros(),
            uv2: Vector2::zeros(),
        }
    }

    #[test]
    fn test_bounding() {
        let triangles = [
            triangle(
                Vector3::new(1.0, 1.0, 0.0),
                Vector3::new(1.0, 1.0, 1.0),
                Vector3::new(0.0, 0.0, 0.0),
            ),
            triangle(
                Vector3::new(-1.0, -1.0, 0.0),
                Vector3::new(-1.0, -1.0, -1.0),
                Vector3::new(0.0, 0.0, 0.0),
            ),
        ];

        let actual = bounding(&triangles);

        let expected = Aabb {
            center: Vector3::new(0.0, 0.0, 0.0),
            half_size: Vector3::new(1.0, 1.0, 1.0)
        };
        assert_eq!(actual, expected);
    }
}
