use nalgebra::Vector3;
use smallvec::SmallVec;

use super::{aabb::Aabb, aap::Aap, intersect::intersect_ray_aap, ray::Ray, triangle::Triangle};

/// Clip Triangle against AABB.
///
/// Implements the Sutherland-Hodgman algorithm.
pub fn clip_triangle_aabb(triangle: &Triangle, aabb: &Aabb) -> SmallVec<[Vector3<f32>; 18]> {
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

    let mut output = SmallVec::<[Vector3<f32>; 18]>::new();
    output.push(triangle.v1);
    output.push(triangle.v2);
    output.push(triangle.v0);

    for clip_plane @ (_, plane) in clip_planes {
        let input = output.clone();
        output.clear();
        for (i, b) in input.iter().enumerate() {
            let a = input[(i as isize - 1).rem_euclid(input.len() as isize) as usize];
            let ray = Ray::between(&a, b);
            let intersecting = intersect_ray_aap(&ray, &plane).map(|t| ray.param(t));
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
mod tests_clip_triangle_aabb {
    use super::*;
    use smallvec::smallvec;

    #[test]
    pub fn triangle_completely_enclosed_in_box() {
        let triangle = Triangle {
            v0: Vector3::new(1.0, 1.0, 1.0),
            v1: Vector3::new(2.0, 1.0, 1.0),
            v2: Vector3::new(2.0, 2.0, 1.0),
        };
        let aabb = Aabb::from_extents(Vector3::new(0.0, 0.0, 0.0), Vector3::new(3.0, 3.0, 3.0));

        let actual = clip_triangle_aabb(&triangle, &aabb);

        let expected: SmallVec<[_; 3]> = smallvec![triangle.v1, triangle.v2, triangle.v0];
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn triangle_above_box() {
        let triangle = Triangle {
            v0: Vector3::new(0.0, 2.0, 0.0),
            v1: Vector3::new(1.0, 2.0, 0.0),
            v2: Vector3::new(1.0, 2.0, 1.0),
        };
        let aabb = Aabb::from_extents(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        let actual = clip_triangle_aabb(&triangle, &aabb);

        let expected: SmallVec<[Vector3<f32>; 3]> = smallvec![];
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn triangle_below_box() {
        let triangle = Triangle {
            v0: Vector3::new(0.0, -1.0, 0.0),
            v1: Vector3::new(1.0, -1.0, 0.0),
            v2: Vector3::new(1.0, -1.0, 1.0),
        };
        let aabb = Aabb::from_extents(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        let actual = clip_triangle_aabb(&triangle, &aabb);

        let expected: SmallVec<[Vector3<f32>; 3]> = smallvec![];
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn triangle_in_zplane_with_all_edges_intersecting_box_sides() {
        let triangle = Triangle {
            v0: Vector3::new(0.0, 0.0, 0.0),
            v1: Vector3::new(12.0, 0.0, 0.0),
            v2: Vector3::new(6.0, 6.0, 0.0),
        };
        let aabb = Aabb::from_extents(Vector3::new(2.0, -1.0, 0.0), Vector3::new(10.0, 4.0, 0.0));

        let actual = clip_triangle_aabb(&triangle, &aabb);

        let expected: SmallVec<[Vector3<f32>; 3]> = smallvec![
            Vector3::new(2.0, 0.0, 0.0),
            Vector3::new(10.0, 0.0, 0.0),
            Vector3::new(10.0, 2.0, 0.0),
            Vector3::new(8.0, 4.0, 0.0),
            Vector3::new(4.0, 4.0, 0.0),
            Vector3::new(2.0, 2.0, 0.0),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn rounding_error_in_ray_param_calculation_example_1() {
        let triangle = Triangle {
            v0: Vector3::new(-1.0, -1.0, -1.0),
            v1: Vector3::new(-1.0, -1.0, 1.0),
            v2: Vector3::new(1.0, -1.0, -1.0),
        };
        let aabb = Aabb::from_extents(
            Vector3::new(-1.5, -1.5012, -1.5),
            Vector3::new(-0.076, 1.5, 1.0),
        );

        let actual = clip_triangle_aabb(&triangle, &aabb);

        let expected: SmallVec<[Vector3<f32>; 0]> = smallvec![];
        let outside = actual
            .into_iter()
            .filter(|p| !aabb.contains(p))
            .collect::<SmallVec<[Vector3<f32>; 1]>>();
        assert_eq!(outside, expected);
    }

    #[test]
    pub fn rounding_error_in_ray_param_calculation_example_2() {
        let triangle = Triangle {
            v0: Vector3::new(-1.0, -1.0, -1.0),
            v1: Vector3::new(-1.0, -1.0, 1.0),
            v2: Vector3::new(1.0, -1.0, -1.0),
        };
        let aabb = Aabb::from_extents(
            Vector3::new(-1.5, -1.5012, -1.5),
            Vector3::new(-0.076, 0.075999975, 0.075999975),
        );

        let actual = clip_triangle_aabb(&triangle, &aabb);

        let expected: SmallVec<[Vector3<f32>; 0]> = smallvec![];
        let outside = actual
            .into_iter()
            .filter(|p| !aabb.contains(p))
            .collect::<SmallVec<[Vector3<f32>; 1]>>();
        assert_eq!(outside, expected);
    }
}
