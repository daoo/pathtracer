use arrayvec::ArrayVec;
use glam::Vec3;

use crate::{aabb::Aabb, aap::Aap, ray::Ray};

/// Clip Triangle against AABB.
///
/// Implements the Sutherland-Hodgman algorithm.
pub fn clip_triangle_aabb(v0: &Vec3, v1: &Vec3, v2: &Vec3, aabb: &Aabb) -> ArrayVec<Vec3, 9> {
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

    let is_inside = |clip_plane: &(bool, Aap), point: Vec3| {
        if clip_plane.0 {
            point[clip_plane.1.axis] <= clip_plane.1.distance
        } else {
            point[clip_plane.1.axis] >= clip_plane.1.distance
        }
    };

    let mut output = ArrayVec::<Vec3, 9>::new();
    unsafe {
        output.push_unchecked(*v1);
        output.push_unchecked(*v2);
        output.push_unchecked(*v0);
    }

    let push_unique = |o: &mut ArrayVec<Vec3, 9>, p: Vec3| {
        if !o.contains(&p) {
            unsafe {
                o.push_unchecked(p);
            }
        }
    };

    for clip_plane in clip_planes {
        if output.is_empty() {
            return output;
        }
        // Default implementation of clone() for arrayvec uses extend which causes unnecessary
        // stack usage and potential calls to panic.
        //
        // I tried to put this into a separate function but that trips up the optimizer, generating
        // much worse code (empirically 20% slower).
        let mut input = ArrayVec::<Vec3, 9>::new();
        for x in &output {
            unsafe {
                input.push_unchecked(*x);
            }
        }
        output.clear();
        let points_iter = input.iter().cycle().skip(input.len() - 1).zip(input.iter());
        for (a, b) in points_iter {
            let ray = Ray::between(*a, *b);
            // When a and b are on different sides of the clip plane it is safe to assume that the
            // denominator in the division in the intersection calculation is non-zero. Hence the
            // unchecked unwraps below.
            let intersecting = clip_plane.1.intersect_ray_point(&ray);
            if is_inside(&clip_plane, *b) {
                if !is_inside(&clip_plane, *a) {
                    push_unique(&mut output, unsafe { intersecting.unwrap_unchecked() });
                }
                push_unique(&mut output, *b);
            } else if is_inside(&clip_plane, *a) {
                push_unique(&mut output, unsafe { intersecting.unwrap_unchecked() });
            }
        }
    }

    output.iter_mut().for_each(|p| *p = aabb.clamp(*p));
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_triangle_completely_enclosed_in_box() {
        let v0 = Vec3::new(1.0, 1.0, 1.0);
        let v1 = Vec3::new(2.0, 1.0, 1.0);
        let v2 = Vec3::new(2.0, 2.0, 1.0);
        let aabb = Aabb::from_extents(Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 3.0, 3.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected = [v1, v2, v0];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_triangle_in_box_side() {
        let v0 = Vec3::new(1.0, 1.0, 0.0);
        let v1 = Vec3::new(2.0, 1.0, 0.0);
        let v2 = Vec3::new(2.0, 2.0, 0.0);
        let aabb = Aabb::from_extents(Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 3.0, 3.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected = [v1, v2, v0];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_triangle_intersecting_flat_box_fully() {
        let v0 = Vec3::new(1.0, 1.0, 0.0);
        let v1 = Vec3::new(2.0, 1.0, 0.0);
        let v2 = Vec3::new(2.0, 2.0, 0.0);
        let aabb = Aabb::from_extents(Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 3.0, 0.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected = [v1, v2, v0];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_triangle_intersecting_flat_box_in_a_line() {
        let v0 = Vec3::new(1.0, 1.0, -1.0);
        let v1 = Vec3::new(2.0, 1.0, 0.0);
        let v2 = Vec3::new(2.0, 2.0, 1.0);
        let aabb = Aabb::from_extents(Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 3.0, 0.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected = [Vec3::new(2.0, 1.0, 0.0), Vec3::new(1.5, 1.5, 0.0)];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_triangle_above_box() {
        let v0 = Vec3::new(0.0, 2.0, 0.0);
        let v1 = Vec3::new(1.0, 2.0, 0.0);
        let v2 = Vec3::new(1.0, 2.0, 1.0);
        let aabb = Aabb::from_extents(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected: &[Vec3] = &[];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_triangle_below_box() {
        let v0 = Vec3::new(0.0, -1.0, 0.0);
        let v1 = Vec3::new(1.0, -1.0, 0.0);
        let v2 = Vec3::new(1.0, -1.0, 1.0);
        let aabb = Aabb::from_extents(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected: &[Vec3] = &[];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_triangle_in_zplane_with_all_edges_intersecting_box_sides() {
        let v0 = Vec3::new(0.0, 0.0, 0.0);
        let v1 = Vec3::new(12.0, 0.0, 0.0);
        let v2 = Vec3::new(6.0, 6.0, 0.0);
        let aabb = Aabb::from_extents(Vec3::new(2.0, -1.0, 0.0), Vec3::new(10.0, 4.0, 0.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected = [
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(10.0, 0.0, 0.0),
            Vec3::new(10.0, 2.0, 0.0),
            Vec3::new(8.0, 4.0, 0.0),
            Vec3::new(4.0, 4.0, 0.0),
            Vec3::new(2.0, 2.0, 0.0),
        ];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_rounding_error_in_ray_param_calculation_example_1() {
        let v0 = Vec3::new(-1.0, -1.0, -1.0);
        let v1 = Vec3::new(-1.0, -1.0, 1.0);
        let v2 = Vec3::new(1.0, -1.0, -1.0);
        let aabb = Aabb::from_extents(Vec3::new(-1.5, -1.5012, -1.5), Vec3::new(-0.076, 1.5, 1.0));

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected: &[Vec3] = &[];
        let outside = actual
            .into_iter()
            .filter(|p| !aabb.contains(*p))
            .collect::<ArrayVec<Vec3, 1>>();
        assert_eq!(outside.as_slice(), expected);
    }

    #[test]
    fn clip_rounding_error_in_ray_param_calculation_example_2() {
        let v0 = Vec3::new(-1.0, -1.0, -1.0);
        let v1 = Vec3::new(-1.0, -1.0, 1.0);
        let v2 = Vec3::new(1.0, -1.0, -1.0);
        let aabb = Aabb::from_extents(
            Vec3::new(-1.5, -1.5012, -1.5),
            Vec3::new(-0.076, 0.075999975, 0.075999975),
        );

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected: &[Vec3] = &[];
        let outside = actual
            .into_iter()
            .filter(|p| !aabb.contains(*p))
            .collect::<ArrayVec<Vec3, 1>>();
        assert_eq!(outside.as_slice(), expected);
    }

    #[test]
    fn clip_incorrect_clamping_if_points_are_outside_two_or_more_clip_planes() {
        let v0 = Vec3::new(3.835834, 0.136162, -3.724971);
        let v2 = Vec3::new(3.836198, 0.135679, -4.556344);
        let v1 = Vec3::new(3.952836, 0.369915, -4.555017);
        let aabb = Aabb::from_extents(
            Vec3::new(3.8359935, 0.241052, -4.272935),
            Vec3::new(3.901177, 0.274277, -4.089322),
        );

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected = [
            Vec3::new(3.901177, 0.2665847, -4.272935),
            Vec3::new(3.8884628, 0.241052, -4.272935),
            Vec3::new(3.8883352, 0.241052, -4.0974307),
            Vec3::new(3.901177, 0.2667079, -4.1885333),
        ];
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn clip_maximum_array_vec_capacity() {
        let v0 = Vec3::new(1.8820591, 1.8356464, -0.024532795);
        let v1 = Vec3::new(0.6663188, 0.14820933, 0.0025525093);
        let v2 = Vec3::new(-0.74280226, -0.56288826, 1.474189);
        let aabb = Aabb::from_extents(Vec3::ZERO, Vec3::ONE);

        let actual = clip_triangle_aabb(&v0, &v1, &v2, &aabb);

        let expected = [
            Vec3::new(1.0, 1.0, 0.44480032),
            Vec3::new(1.0, 0.61557496, 0.0),
            Vec3::new(0.78088975, 0.30723286, 0.0),
            Vec3::new(0.6663188, 0.14820933, 0.0025525093),
            Vec3::new(0.37262508, 0.0, 0.3092759),
            Vec3::new(0.0, 0.0, 0.9160062),
            Vec3::new(0.0, 0.07259283, 1.0),
            Vec3::new(0.08769246, 0.19599774, 1.0),
            Vec3::new(0.9675606, 1.0, 0.49762005),
        ];
        assert_eq!(actual.as_slice(), expected);
    }
}
