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
    use nalgebra::vector;

    #[test]
    fn test_bounding() {
        let triangles = [
            Triangle{ v0: vector![1.0, 1.0, 0.0], v1: vector![1.0, 1.0, 1.0], v2: vector![0.0, 0.0, 0.0] },
            Triangle{ v0: vector![-1.0, -1.0, 0.0], v1: vector![-1.0, -1.0, -1.0], v2: vector![0.0, 0.0, 0.0] },
        ];

        let actual = bounding(&triangles);

        let expected = Aabb {
            center: vector![0.0, 0.0, 0.0],
            half_size: vector![1.0, 1.0, 1.0]
        };
        assert_eq!(actual, expected);
    }
}
