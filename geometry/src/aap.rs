use glam::{Vec2, Vec3};

use crate::ray::Ray;

use super::axis::Axis;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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

    pub fn add_to(&self, point: Vec2) -> Vec3 {
        self.axis.add_to(point, self.distance)
    }

    pub fn vector(&self) -> Vec3 {
        self.axis.as_vector3(self.distance)
    }

    #[inline]
    pub fn intersect_ray(&self, ray: &Ray) -> Option<f32> {
        let denom = ray.direction[self.axis];
        if denom == 0.0 {
            // Ray parallel to plane (ray direction vector orthogonal to plane vector).
            return None;
        }
        Some((self.distance - ray.origin[self.axis]) / denom)
    }

    pub fn intersect_ray_point(&self, ray: &Ray) -> Option<Vec3> {
        self.intersect_ray(ray).map(|t| match self.axis {
            Axis::X => Vec3::new(
                self.distance,
                ray.origin.y + t * ray.direction.y,
                ray.origin.z + t * ray.direction.z,
            ),
            Axis::Y => Vec3::new(
                ray.origin.x + t * ray.direction.x,
                self.distance,
                ray.origin.z + t * ray.direction.z,
            ),
            Axis::Z => Vec3::new(
                ray.origin.x + t * ray.direction.x,
                ray.origin.y + t * ray.direction.y,
                self.distance,
            ),
        })
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
        let ray = Ray::between(Vec3::ZERO, Vec3::new(5.0, 0.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(1.0));
    }

    #[test]
    fn intersect_ray_from_origo_to_beyond_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_ray_from_origo_to_short_of_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(1.25));
    }

    #[test]
    fn intersect_ray_from_just_before_plane_to_beyond_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(Vec3::new(4.0, 0.0, 0.0), Vec3::new(6.0, 0.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_ray_from_just_after_plane_to_before_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 5.0,
        };
        let ray = Ray::between(Vec3::new(6.0, 0.0, 0.0), Vec3::new(4.0, 0.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_non_axis_aligned_ray_through_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 2.0,
        };
        let ray = Ray::between(Vec3::new(1.0, 1.0, 0.0), Vec3::new(3.0, 3.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_non_axis_aligned_ray_with_positive_direction_through_plane() {
        let plane = Aap {
            axis: Axis::Y,
            distance: 2.0,
        };
        let ray = Ray::between(Vec3::new(1.0, 1.0, 0.0), Vec3::new(3.0, 3.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_non_axis_aligned_ray_with_negative_direction_through_plane() {
        let plane = Aap {
            axis: Axis::Y,
            distance: 2.0,
        };
        let ray = Ray::between(Vec3::new(3.0, 1.0, 0.0), Vec3::new(1.0, 3.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), Some(0.5));
    }

    #[test]
    fn intersect_ray_parallel_to_plane() {
        let plane = Aap {
            axis: Axis::X,
            distance: 2.0,
        };
        let ray = Ray::between(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), None);
    }

    #[test]
    fn intersect_ray_point_through_plane_where_result_differs_in_xyz() {
        let plane = Aap {
            axis: Axis::Y,
            distance: 2.0,
        };
        let ray = Ray::between(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 4.0, 6.0));

        assert_eq!(
            plane.intersect_ray_point(&ray),
            Some(Vec3::new(1.0, 2.0, 3.0))
        );
    }
}
