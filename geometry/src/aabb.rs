use nalgebra::Vector3;

use super::aap::Aap;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Aabb {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl Aabb {
    pub fn from_extents(min: Vector3<f32>, max: Vector3<f32>) -> Aabb {
        debug_assert!(min <= max);
        Aabb { min, max }
    }

    pub fn empty() -> Aabb {
        Aabb {
            min: Vector3::zeros(),
            max: Vector3::zeros(),
        }
    }

    pub fn unit() -> Aabb {
        Aabb {
            min: Vector3::new(0., 0., 0.),
            max: Vector3::new(1., 1., 1.),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min == self.max
    }

    pub fn center(&self) -> Vector3<f32> {
        self.min + self.half_size()
    }

    pub fn half_size(&self) -> Vector3<f32> {
        self.size() / 2.0
    }

    pub fn size(&self) -> Vector3<f32> {
        self.max - self.min
    }

    pub fn min(&self) -> Vector3<f32> {
        self.min
    }

    pub fn max(&self) -> Vector3<f32> {
        self.max
    }

    pub fn surface_area(&self) -> f32 {
        let size = self.size();
        2. * (size.x * size.y + size.x * size.z + size.y * size.z)
    }

    pub fn volume(&self) -> f32 {
        let size = self.size();
        size.x * size.y * size.z
    }

    #[must_use]
    pub fn enlarge(&self, delta: &Vector3<f32>) -> Aabb {
        let half_delta = delta / 2.0;
        Aabb {
            min: self.min - half_delta,
            max: self.max + half_delta,
        }
    }

    pub fn split(&self, plane: &Aap) -> (Aabb, Aabb) {
        let mut new_max = self.max;
        new_max[plane.axis] = plane.distance;

        let mut new_min = self.min;
        new_min[plane.axis] = plane.distance;

        let fst = Aabb::from_extents(self.min, new_max);
        let snd = Aabb::from_extents(new_min, self.max);
        (fst, snd)
    }

    pub fn clamp(&self, point: Vector3<f32>) -> Vector3<f32> {
        Vector3::new(
            point.x.clamp(self.min.x, self.max.x),
            point.y.clamp(self.min.y, self.max.y),
            point.z.clamp(self.min.z, self.max.z),
        )
    }

    pub fn contains(&self, point: &Vector3<f32>) -> bool {
        *point >= self.min && *point <= self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_in_half_halves_the_volume() {
        let aabb = Aabb::unit();

        let actual = aabb.split(&Aap::new_x(0.5));

        assert_eq!(aabb.volume(), 1.0);
        assert_eq!(actual.0.volume(), 0.5);
        assert_eq!(actual.1.volume(), 0.5);
    }
}
