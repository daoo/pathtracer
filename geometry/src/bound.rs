use crate::{aabb::Aabb, triangle::Triangle};

pub fn triangles_bounding_box(triangles: &[Triangle]) -> Aabb {
    if triangles.is_empty() {
        return Aabb::empty();
    }
    let mut a = triangles[0].min();
    let mut b = triangles[0].max();
    for triangle in triangles {
        a = a.inf(&triangle.min());
        b = b.sup(&triangle.max());
    }
    Aabb::from_extents(a, b)
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;

    use super::*;

    #[test]
    fn two_triangles_give_expected_min_max() {
        let triangles = [
            Triangle {
                v0: Vector3::new(1., 1., 0.),
                v1: Vector3::new(1., 1., 1.),
                v2: Vector3::new(0., 0., 0.),
            },
            Triangle {
                v0: Vector3::new(-1., -1., 0.),
                v1: Vector3::new(-1., -1., -1.),
                v2: Vector3::new(0., 0., 0.),
            },
        ];

        let actual = triangles_bounding_box(&triangles);

        let expected = Aabb::from_extents(Vector3::new(-1., -1., -1.), Vector3::new(1., 1., 1.));
        assert_eq!(actual, expected);
    }
}
