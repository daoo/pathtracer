use glam::Vec3;

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn between(a: Vec3, b: Vec3) -> Self {
        Self {
            origin: a,
            direction: b - a,
        }
    }

    pub fn extended(&self, t: f32) -> Self {
        Self {
            origin: self.origin,
            direction: t * self.direction,
        }
    }

    pub fn param(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn reverse(&self) -> Self {
        Self {
            origin: self.origin + self.direction,
            direction: -self.direction,
        }
    }
}
