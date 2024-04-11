use crate::{aabb::Aabb, Geometry};

pub fn combine_bounding_boxes(a: &Aabb, b: &Aabb) -> Aabb {
    Aabb::from_extents(a.min().inf(&b.min()), a.max().sup(&b.max()))
}

pub fn geometries_bounding_box<G>(geometries: &[G]) -> Aabb
where
    G: Geometry,
{
    if geometries.is_empty() {
        return Aabb::empty();
    }
    let mut a = geometries[0].min();
    let mut b = geometries[0].max();
    for triangle in geometries {
        a = a.inf(&triangle.min());
        b = b.sup(&triangle.max());
    }
    Aabb::from_extents(a, b)
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;

    use crate::triangle::Triangle;

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

        let actual = geometries_bounding_box(&triangles);

        let expected = Aabb::from_extents(Vector3::new(-1., -1., -1.), Vector3::new(1., 1., 1.));
        assert_eq!(actual, expected);
    }
}
