use arrayvec::ArrayVec;
use nalgebra::Vector3;

use crate::{aabb::Aabb, aap::Aap, ray::Ray};

/// Clip Triangle against AABB.
///
/// Implements the Sutherland-Hodgman algorithm.
pub fn clip_triangle_aabb(
    v0: &Vector3<f32>,
    v1: &Vector3<f32>,
    v2: &Vector3<f32>,
    aabb: &Aabb,
) -> ArrayVec<Vector3<f32>, 6> {
    let aabb_min = aabb.min();
    let aabb_max = aabb.max();
    let clip_planes = [
        (false, Aap::new_x(aabb_min.x)),
        (false, Aap::new_y(aabb_min.y)),
        (false, Aap::new_z(aabb_min.z)),
        (true, Aap::new_x(aabb_max.x)),
        (true, Aap::new_y(aabb_max.y)),
        (true, Aap::new_z(aabb_max.z)),
    ];

    let is_inside = |clip_plane: &(bool, Aap), point: &Vector3<f32>| {
        if clip_plane.0 {
            point[clip_plane.1.axis] <= clip_plane.1.distance
        } else {
            point[clip_plane.1.axis] >= clip_plane.1.distance
        }
    };

    let mut output = ArrayVec::<Vector3<f32>, 6>::new();
    output.push(*v1);
    output.push(*v2);
    output.push(*v0);

    for clip_plane @ (_, plane) in clip_planes {
        let input = output.clone();
        output.clear();
        for (i, b) in input.iter().enumerate() {
            let a = input[if i == 0 { input.len() - 1 } else { i - 1 }];
            let ray = Ray::between(&a, b);
            let intersecting = plane.intersect_ray_point(&ray);
            if is_inside(&clip_plane, b) {
                if !is_inside(&clip_plane, &a) {
                    output.push(aabb.clamp(intersecting.unwrap()));
                }
                output.push(*b);
            } else if is_inside(&clip_plane, &a) {
                output.push(aabb.clamp(intersecting.unwrap()));
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_triangle_completely_enclosed_in_box() {
        let v0 = Vector3::new(1.0, 1.0, 1.0);
        let v1 = Vector3::new(2.0, 1.0, 1.0);
        let v2 = Vector3::new(2.0, 2.0, 1.0);
        let aabb = Aabb::from_extents(Vector3::new(0.0, 0.0, 0.0), Vector3::new(3.0, 3.0, 3.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected = [v1, v2, v0];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_triangle_above_box() {
        let v0 = Vector3::new(0.0, 2.0, 0.0);
        let v1 = Vector3::new(1.0, 2.0, 0.0);
        let v2 = Vector3::new(1.0, 2.0, 1.0);
        let aabb = Aabb::from_extents(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected: &[Vector3<f32>] = &[];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_triangle_below_box() {
        let v0 = Vector3::new(0.0, -1.0, 0.0);
        let v1 = Vector3::new(1.0, -1.0, 0.0);
        let v2 = Vector3::new(1.0, -1.0, 1.0);
        let aabb = Aabb::from_extents(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected: &[Vector3<f32>] = &[];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_triangle_in_zplane_with_all_edges_intersecting_box_sides() {
        let v0 = Vector3::new(0.0, 0.0, 0.0);
        let v1 = Vector3::new(12.0, 0.0, 0.0);
        let v2 = Vector3::new(6.0, 6.0, 0.0);
        let aabb = Aabb::from_extents(Vector3::new(2.0, -1.0, 0.0), Vector3::new(10.0, 4.0, 0.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected = [
            Vector3::new(2.0, 0.0, 0.0),
            Vector3::new(10.0, 0.0, 0.0),
            Vector3::new(10.0, 2.0, 0.0),
            Vector3::new(8.0, 4.0, 0.0),
            Vector3::new(4.0, 4.0, 0.0),
            Vector3::new(2.0, 2.0, 0.0),
        ];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_rounding_error_in_ray_param_calculation_example_1() {
        let v0 = Vector3::new(-1.0, -1.0, -1.0);
        let v1 = Vector3::new(-1.0, -1.0, 1.0);
        let v2 = Vector3::new(1.0, -1.0, -1.0);
        let aabb = Aabb::from_extents(
            Vector3::new(-1.5, -1.5012, -1.5),
            Vector3::new(-0.076, 1.5, 1.0),
        );

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected: &[Vector3<f32>] = &[];
        let outside = actual
            .into_iter()
            .filter(|p| !aabb.contains(p))
            .collect::<ArrayVec<Vector3<f32>, 1>>();
        assert_eq!(outside.as_slice(), expected);
    }

    #[test]
    fn clip_rounding_error_in_ray_param_calculation_example_2() {
        let v0 = Vector3::new(-1.0, -1.0, -1.0);
        let v1 = Vector3::new(-1.0, -1.0, 1.0);
        let v2 = Vector3::new(1.0, -1.0, -1.0);
        let aabb = Aabb::from_extents(
            Vector3::new(-1.5, -1.5012, -1.5),
            Vector3::new(-0.076, 0.075999975, 0.075999975),
        );

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected: &[Vector3<f32>] = &[];
        let outside = actual
            .into_iter()
            .filter(|p| !aabb.contains(p))
            .collect::<ArrayVec<Vector3<f32>, 1>>();
        assert_eq!(outside.as_slice(), expected);
    }
}
