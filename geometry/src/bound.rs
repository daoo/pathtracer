use crate::{aabb::Aabb, shape::Shape};

pub fn combine_bounding_boxes(a: &Aabb, b: &Aabb) -> Aabb {
    Aabb::from_extents(a.min().min(*b.min()), a.max().max(*b.max()))
}

pub fn geometries_bounding_box(shape: &[Shape]) -> Aabb {
    let mut it = shape.iter();
    let (mut a, mut b) = if let Some(first) = it.next() {
        (first.min(), first.max())
    } else {
        return Aabb::empty();
    };
    while let Some(s) = it.next() {
        a = a.min(s.min());
        b = b.max(s.max());
    }
    Aabb::from_extents(a, b)
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::triangle::Triangle;

    use super::*;

    #[test]
    fn two_triangles_give_expected_min_max() {
        let triangles = [
            Triangle {
                v0: Vec3::new(1., 1., 0.),
                v1: Vec3::new(1., 1., 1.),
                v2: Vec3::new(0., 0., 0.),
            },
            Triangle {
                v0: Vec3::new(-1., -1., 0.),
                v1: Vec3::new(-1., -1., -1.),
                v2: Vec3::new(0., 0., 0.),
            },
        ]
        .map(|t| t.into());

        let actual = geometries_bounding_box(&triangles);

        let expected = Aabb::from_extents(Vec3::new(-1., -1., -1.), Vec3::new(1., 1., 1.));
        assert_eq!(actual, expected);
    }
}
