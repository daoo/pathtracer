use nalgebra::{Vector2, Vector3};

use crate::ray::Ray;

use super::axis::Axis;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Aap {
    pub axis: Axis,
    pub distance: f32,
}

impl Aap {
    pub fn new_x(distance: f32) -> Aap {
        Aap {
            axis: Axis::X,
            distance,
        }
    }

    pub fn new_y(distance: f32) -> Aap {
        Aap {
            axis: Axis::Y,
            distance,
        }
    }

    pub fn new_z(distance: f32) -> Aap {
        Aap {
            axis: Axis::Z,
            distance,
        }
    }

    pub fn add_to(&self, point: Vector2<f32>) -> Vector3<f32> {
        self.axis.add_to(point, self.distance)
    }

    pub fn vector(&self) -> Vector3<f32> {
        self.axis.as_vector3(self.distance)
    }

    pub fn total_cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.axis
            .cmp(&other.axis)
            .then(f32::total_cmp(&self.distance, &other.distance))
    }

    pub fn intersect_ray(&self, ray: &Ray) -> Option<f32> {
        let denom = ray.direction[self.axis];
        if denom == 0.0 {
            // Ray parallel to plane (ray direction vector orthogonal to plane vector).
            return None;
        }
        Some((self.distance - ray.origin[self.axis]) / denom)
    }
}

#[cfg(test)]
mod tests {
    use crate::axis::Axis;

    use super::*;

    #[test]
    fn intersect_ray_from_origo_to_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::zeros(), &Vector3::new(5.0, 0.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(1.0));
    }

    #[test]
    fn intersect_ray_from_origo_to_beyond_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::zeros(), &Vector3::new(10.0, 0.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_ray_from_origo_to_short_of_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::zeros(), &Vector3::new(4.0, 0.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(1.25));
    }

    #[test]
    fn intersect_ray_from_just_before_plane_to_beyond_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::new(4.0, 0.0, 0.0), &Vector3::new(6.0, 0.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_ray_from_just_after_plane_to_before_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(&Vector3::new(6.0, 0.0, 0.0), &Vector3::new(4.0, 0.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_non_axis_aligned_ray_through_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 2.0,
        };
        let ray = Ray::between(&Vector3::new(1.0, 1.0, 0.0), &(Vector3::new(3.0, 3.0, 0.0)));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_non_axis_aligned_ray_with_positive_direction_through_plane() {
        let plane = Aap {
            axis: Axis::Y,
            distance: 2.0,
        };
        let ray = Ray::between(&Vector3::new(1.0, 1.0, 0.0), &Vector3::new(3.0, 3.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_non_axis_aligned_ray_with_negative_direction_through_plane() {
        let plane = Aap {
            axis: Axis::Y,
            distance: 2.0,
        };
        let ray = Ray::between(&Vector3::new(3.0, 1.0, 0.0), &Vector3::new(1.0, 3.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_ray_parallel_to_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 2.0,
        };
        let ray = Ray::between(&Vector3::new(0.0, 0.0, 0.0), &(Vector3::new(0.0, 1.0, 0.0)));

        assert_eq!(plane.intersect_ray(&ray), None);
    }
}
