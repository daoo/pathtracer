use glam::Vec3;

use super::aap::Aap;

#[derive(Clone, Debug, PartialEq)]
pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    pub fn from_extents(min: Vec3, max: Vec3) -> Aabb {
        debug_assert!(min.cmple(max).all());
        Aabb { min, max }
    }

    pub fn empty() -> Aabb {
        Aabb {
            min: Vec3::ZERO,
            max: Vec3::ZERO,
        }
    }

    pub fn unit() -> Aabb {
        Aabb {
            min: Vec3::new(0., 0., 0.),
            max: Vec3::new(1., 1., 1.),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min == self.max
    }

    pub fn center(&self) -> Vec3 {
        self.min + self.half_size()
    }

    pub fn half_size(&self) -> Vec3 {
        self.size() / 2.0
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn min(&self) -> &Vec3 {
        &self.min
    }

    pub fn max(&self) -> &Vec3 {
        &self.max
    }

    pub fn surface_area(&self) -> f32 {
        let size = self.size();
        2. * (size.x * size.y + size.x * size.z + size.y * size.z)
    }

    pub fn volume(&self) -> f32 {
        let size = self.size();
        size.x * size.y * size.z
    }

    pub fn enlarge(&self, delta: Vec3) -> Aabb {
        let half_delta = delta / 2.0;
        Aabb {
            min: self.min - half_delta,
            max: self.max + half_delta,
        }
    }

    pub fn sides(&self) -> [Aap; 6] {
        [
            Aap::new_x(self.min.x),
            Aap::new_x(self.max.x),
            Aap::new_y(self.min.y),
            Aap::new_y(self.max.y),
            Aap::new_z(self.min.z),
            Aap::new_z(self.max.z),
        ]
    }

    #[inline]
    pub fn split(&self, plane: &Aap) -> (Aabb, Aabb) {
        debug_assert!(plane.distance >= self.min[plane.axis]);
        debug_assert!(plane.distance <= self.max[plane.axis]);

        let mut new_max = self.max;
        new_max[plane.axis] = plane.distance;

        let mut new_min = self.min;
        new_min[plane.axis] = plane.distance;

        let fst = Aabb::from_extents(self.min, new_max);
        let snd = Aabb::from_extents(new_min, self.max);
        (fst, snd)
    }

    pub fn clamp(&self, point: Vec3) -> Vec3 {
        let clamp = |x, a, b| {
            if x < a {
                a
            } else if x > b {
                b
            } else {
                x
            }
        };
        Vec3::new(
            clamp(point.x, self.min.x, self.max.x),
            clamp(point.y, self.min.y, self.max.y),
            clamp(point.z, self.min.z, self.max.z),
        )
    }

    pub fn contains(&self, point: Vec3) -> bool {
        point.cmpge(self.min).all() && point.cmple(self.max).all()
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
