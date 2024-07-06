use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn between(a: Vec3, b: Vec3) -> Ray {
        Ray {
            origin: a,
            direction: b - a,
        }
    }

    pub fn extended(&self, t: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: t * self.direction,
        }
    }

    pub fn param(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn reverse(&self) -> Ray {
        Ray {
            origin: self.origin + self.direction,
            direction: -self.direction,
        }
    }
}
